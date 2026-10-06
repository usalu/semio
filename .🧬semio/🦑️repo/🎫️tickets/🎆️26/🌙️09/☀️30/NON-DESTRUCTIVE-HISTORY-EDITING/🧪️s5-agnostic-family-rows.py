#!/usr/bin/env python3
"""🧾️ S5-AGNOSTIC: turns one family run of `🧪️s5-agnostic-run-family.sh` into one row per crate of
`🗑️generated/s5-agnostic/acceptance-results.tsv` (time, crate, exit, class, G12, inputs, reload, child, payload pass/fail, summary).
A crate with a `Running unittests` section in the test log is judged by its own section (PASS / FAIL / ABORT / NO-LAW); a crate the
build log names in `could not compile` is COMPILE-RED with its first `file:line: error` lines (attributed by its artifact directory);
any other crate is NOT-RUN, naming the crate that blocked it. A PASS removes that crate's lib-test executable under its unit `.lock`
(non-blocking, the disk guard's protocol) unless keep is `1`. Usage: <family> <build-log> <test-log> <exit> <keep> <crate>…"""
import fcntl, json, os, re, shutil, sys, time

T = os.path.dirname(os.path.abspath(__file__))
G = os.path.join(T, "🗑️generated", "s5-agnostic")


def read(path):
    return open(path, encoding="utf-8", errors="replace").read() if os.path.isfile(path) else ""


def errors_of(build, directory):
    lines = [line for line in build.splitlines() if re.search(r":\d+:\d+: error", line)]
    own = [line for line in lines if directory and directory in line]
    return own, lines


def judge(section):
    lines = section.splitlines()
    names = re.findall(r"^test (\S+) \.\.\. ", section, flags=re.M)
    block = re.search(r"^failures:\n((?:    \S+\n)+)\ntest result", section, flags=re.M)
    failed = {name.strip() for name in block.group(1).splitlines()} if block else set()
    result = re.search(r"^test result: (\w+)\. (\d+) passed; (\d+) failed", section, flags=re.M)

    def law(segment):
        hits = [name for name in names if name.rsplit("::", 1)[-1] == segment]
        if not hits:
            return "-"
        if not result:
            return "ABORT"
        return "FAIL" if any(name in failed for name in hits) else "ok"

    laws = [law("history_edits_end_to_end"), law("history_edit_inputs_resolve"), law("documents_reload_identically"), law("child_history_edits_end_to_end")]
    payload = [name for name in names if "semio_payload_law" in name]
    payload_failed = sum(1 for name in payload if name in failed)
    panics = []
    for index, line in enumerate(lines):
        if "panicked at" in line:
            body = []
            for follow in lines[index + 1:index + 8]:
                if not follow.strip() or follow.startswith("note:") or follow.startswith("stack backtrace"):
                    break
                body.append(follow.strip())
            panics.append((line.split("panicked at")[-1].strip()[:160] + " " + " ".join(body))[:900])
    summaries = [line[line.index("["):] for line in lines if "[history-edit-" in line or "[documents-reload]" in line or "[child-history-edit]" in line]
    if not result:
        return "ABORT", laws, payload, payload_failed, "the test process died without a result after " + (names[-1] if names else "no test") + " | " + " | ".join(lines[-4:])
    wired = [status for status in laws if status != "-"]
    kind = "NO-LAW" if not wired else ("PASS" if all(status == "ok" for status in wired) else "FAIL")
    return kind, laws, payload, payload_failed, " | ".join(summaries + panics)


def main():
    family, build_path, test_path, code, keep = sys.argv[1:6]
    crates = sys.argv[6:]
    build, test = read(build_path), read(test_path)
    census = {row["crate"]: row for row in json.load(open(os.path.join(G, "census.json"), encoding="utf-8"))}
    parts = re.split(r"^\s+Running unittests .*?\((.*?/out/([a-z0-9_]+)-[0-9a-f]+)\)\s*$", test, flags=re.M)
    sections = {parts[index + 1]: (parts[index], parts[index + 2]) for index in range(1, len(parts) - 2, 3)}
    red = list(dict.fromkeys(re.findall(r"could not compile `([^`]+)` \(lib", build + "\n" + test)))
    stamp = time.strftime("%F %T")
    with open(os.path.join(G, "acceptance-results.tsv"), "a", encoding="utf-8") as out:
        for crate in crates:
            row = census.get(crate, {})
            directory = os.path.dirname(os.path.dirname(os.path.dirname(row.get("manifest", "")))) if row else ""
            section = sections.get(crate.replace("-", "_"))
            executable = None
            if section is not None:
                executable, body = section
                kind, laws, payload, payload_failed, summary = judge(body)
                cells = [stamp, crate, "0" if kind == "PASS" else "101", kind, *laws, f"{len(payload) - payload_failed}/{payload_failed}", summary]
            else:
                own, every = errors_of(build + "\n" + test, directory)
                if crate in red:
                    kind, summary = "COMPILE-RED", f"could not compile {crate} (lib test): " + " | ".join((own or every)[:3])
                else:
                    blockers = [name for name in red if name != crate]
                    kind = "NOT-RUN"
                    summary = (f"not run — blocked by {', '.join(blockers[:4])}: " + " | ".join(every[:2])) if blockers else ("not run: " + " | ".join((test or build).splitlines()[-3:]))
                cells = [stamp, crate, "101", kind, "-", "-", "-", "-", "0/0", summary]
            cells[-1] = cells[-1].replace("\t", " ")[:4000]
            out.write("\t".join(cells) + "\n")
            print(family, crate, cells[3], "/".join(cells[4:8]), cells[8])
            if executable and cells[3] == "PASS" and keep != "1" and os.path.isfile(executable):
                unit = os.path.dirname(os.path.dirname(executable))
                try:
                    handle = os.open(os.path.join(unit, ".lock"), os.O_RDWR | os.O_CREAT)
                    fcntl.flock(handle, fcntl.LOCK_EX | fcntl.LOCK_NB)
                except OSError:
                    continue
                os.remove(executable)
                shutil.rmtree(executable + ".dSYM", ignore_errors=True)
                os.close(handle)


if __name__ == "__main__":
    main()
