"""x-proctor step 5 (AGENTS.md forbids compatibility layers): deletes the in-process `buildProctor`, the transitional `PROCTOR_ACQUISITION_*` switch and the tests, fixtures and prose that only served them.
Apply only after the live verification of x-proctor (cold build, warm hit, `check` consuming the receipt)."""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

TEST = f"{SITE}/🧪️tests/🏗️proctor-build/🟦️.ts"
FIXTURE = "🎓️teaching/🛂️proctor/🏗️bootstrap/🧪️tests/🧫️fixtures/🔣️.json"
README = "🎓️teaching/🛂️proctor/README.md"
IMPORT = re.compile(r'^import (?P<head>[^;\n]*?) from (?P<source>"[^"\n]+")(?P<tail>[^;\n]*);$', re.M)


def prune_imports(text):
    """Drops named and default imports whose identifier no longer appears outside the import lines; side-effect imports stay."""
    body = IMPORT.sub("", text)
    def rewrite(match):
        head = match.group("head")
        if head.startswith("type ") or "*" in head:
            return match.group(0)
        default, _, rest = head.partition("{")
        if "{" not in head:
            name = head.strip()
            return match.group(0) if re.search(rf"\b{re.escape(name)}\b", body) else ""
        names = [part.strip() for part in rest.rsplit("}", 1)[0].split(",") if part.strip()]
        used = [part for part in names if re.search(rf"\b{re.escape(part.split(' as ')[-1].replace('type ', '').strip())}\b", body)]
        keep_default = default.strip().rstrip(",").strip()
        if keep_default and not re.search(rf"\b{re.escape(keep_default)}\b", body):
            keep_default = ""
        if not used and not keep_default:
            return ""
        if used == names and (keep_default or not default.strip()):
            return match.group(0)
        pieces = ([keep_default] if keep_default else []) + (["{ " + ", ".join(used) + " }"] if used else [])
        return f'import {", ".join(pieces)} from {match.group("source")}{match.group("tail")};'
    pruned = IMPORT.sub(rewrite, text)
    return re.sub(r"\n\n\n+", "\n\n", re.sub(r"^\n(?=import )", "", pruned, flags=re.M)) if pruned != text else text


def drop_block(text, start_marker, end_marker):
    start = text.find(start_marker)
    if start < 0:
        return text
    end = text.index(end_marker, start) + len(end_marker)
    return text[:start] + text[end:]


def bootstrap(text):
    if "export async function buildProctor" in text:
        start = text.rindex("\n/** 🏗️ Builds with the original caller", 0, text.index("export async function buildProctor"))
        end = text.index("\n}\n", text.index("export async function buildProctor")) + 3
        text = text[:start] + text[end:]
    text = text.replace(" * {@link buildProctor} returns an owner that callers close after all consumers, including restarts, have drained.", " * {@link ProctorArtifact} is an owner that callers close after all consumers, including restarts, have drained.")
    return prune_imports(text)


def product(text):
    text = re.sub(r"(\bimport \{[^}\n]*?)\bbuildProctor, ", r"\1", text, count=1)
    text = drop_block(text, "/** 📥️ Which way the course obtains the proctor", 'export const PROCTOR_ACQUISITION_ENVIRONMENT = "SEMIO_PROCTOR_ACQUISITION";\n\n')
    text = drop_block(text, "/** 🎛️ The acquisition of this run:", "PROCTOR_ACQUISITION_DEFAULT;\n}\n\n")
    text = text.replace('  if (proctorAcquisition(env) === "build") return buildProctor(repoRoot, invocation, env, signal, requestedMode);\n', "")
    text = text.replace("With the product acquisition it resolves `@teaching/proctor:build`'s receipt for\n * the original build mode", "It resolves `@teaching/proctor:build`'s receipt for\n * the original build mode")
    return text


def test_file(text):
    text = text.replace('for (const row of fixtures.authority) it(`refuses ${row.id} authority before opening compiler storage or launching Cargo`, async () => {\n  await expect(buildProctor(resolve("."), row.invocation as never)).rejects.toThrow();\n});\n\n', "")
    marker = 'it("carries the current compiler phase through capture and drains children and byte exports before closing storage"'
    if marker in text:
        text = text[:text.index(marker)].rstrip("\n") + "\n"
    text = re.sub(r"(\bimport \{[^}\n]*?)\bbuildProctor, ", r"\1", text, count=1)
    return prune_imports(text)


def fixture(text):
    document = json.loads(text)
    for key in ("authority", "lifecycle"):
        document.pop(key, None)
    return json.dumps(document, indent=2, ensure_ascii=False) + ("\n" if text.endswith("\n") else "")


def readme(text):
    text = text.replace("`buildProctor`, `proctorReady`) is what", "`proctorReady`; `🧾️product/🟦️.ts`: `acquireProctor`) is what")
    text = text.replace("`buildProctor`\nreturns an owner with `launch` and `close`; its source inventory comes from bounded, admitted Cargo metadata.", "`acquireProctor`\nreturns an owner with `launch` and `close` over the product `@teaching/proctor:build` published; the source inventory of that build comes from bounded, admitted Cargo metadata.")
    return text


def run(root):
    changed = []
    for rel, edit in ((BOOTSTRAP, bootstrap), (PRODUCT, product), (TEST, test_file), (FIXTURE, fixture), (README, readme)):
        before = read(root, rel)
        after = edit(before)
        if before != after:
            write(root, rel, after)
            changed.append(rel)
    left = [rel for rel in (BOOTSTRAP, PRODUCT, TEST, FIXTURE, README) if re.search(r"buildProctor\b|PROCTOR_ACQUISITION|proctorAcquisition", read(root, rel))]
    if left:
        refuse("cleanup left buildProctor or the acquisition switch in " + ", ".join(left))
    return changed
