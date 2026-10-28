#!/usr/bin/env python3
"""🏗️ Rebuilds the staged batches of lane langpick from the CURRENT live files: every edit is an anchored text transform, so a rebuild after peer landings rebases it.

usage: python3 -I build.py [batch ...] [--root LIVE]
A file that a pending batch of lane `progress` also edits is based on that batch's new content, and every batch of this lane is based on the batches before it (land order: progress umlaut, lang, status, rows; then langpick core, copies, imports, launcher, dashboard, routing).
"""
import hashlib, importlib.util, json, os, shutil, stat, sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
SRC = os.path.join(HERE, "src")
ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(C)))))))
F = "🧰️framework/"
RT = F + "🔨️modules/⚙️runtime/🗣️language/"
OLD = F + "🔨️modules/🏃️process/🏗️build/🗣️language/"
ENTRY = F + "📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts"
PROCESS_ENTRY = F + "🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🟦️.ts"
LIB = F + "🛍️products/🦑️repo/🔨️modules/📚️library/"
PENDING = [("progress-tools", b) for b in ("umlaut", "lang", "status", "rows")]


def sha(data):
    return None if data is None else hashlib.sha256(data).hexdigest()


def pending_base():
    """📚️ Live content, or the new content of a pending batch of another lane that edits the same file first."""
    table = {}
    for tools, batch in PENDING:
        folder = os.path.join(C, tools, "staged", batch)
        if not os.path.exists(os.path.join(folder, "manifest.json")): continue
        for rel, row in json.load(open(os.path.join(folder, "manifest.json"), encoding="utf8")).items():
            live = read_live(rel)
            current = table.get(rel, live)
            if sha(current) == row["base"] and row["new"] is not None:
                table[rel] = open(os.path.join(folder, "files", row["file"]), "rb").read()
    return table


def read_live(rel):
    path = os.path.join(ROOT, rel)
    return open(path, "rb").read() if os.path.exists(path) else None


def src(name):
    return open(os.path.join(SRC, name), "rb").read()


def replace(text, old, new, count=1):
    found = text.count(old)
    if found != count: raise SystemExit(f"anchor found {found}x (want {count}): {old[:120]!r}")
    return text.replace(old, new)


def depth_to_framework(rel):
    return "../" * (len(rel.split("/")) - 2)


def scopes_edit(text):
    data = json.loads(text)
    moved = {OLD + "🔣️.json": RT + "🔣️.json", OLD + "🟦️.ts": RT + "🟦️.ts"}
    hits = 0
    for scope in data["scopes"]:
        for cls in scope["classes"]:
            modules = cls["traced"]["modules"]
            if any(m in moved for m in modules):
                hits += 1
                cls["traced"]["modules"] = sorted({moved.get(m, m) for m in modules}, key=lambda s: s.encode("utf-16-be"))
    if hits == 0: raise SystemExit("scopes: no traced class lists the language module")
    return json.dumps(data, ensure_ascii=False, indent=2) + "\n"


def insert_sorted(text, anchor, new):
    """🔤️ Adds `new` to the block of consecutive negated-pattern lines that holds `anchor`, at its UTF-16 code unit position (the order `buildSchedulingExclusionPatternsV1` sorts by, which a law compares with the policy and the Nx mirror)."""
    lines = text.split("\n")
    at = lines.index(anchor)
    first = last = at
    while lines[first - 1].lstrip().startswith('"!{workspaceRoot}/'): first -= 1
    while lines[last + 1].lstrip().startswith('"!{workspaceRoot}/'): last += 1
    indent = anchor[:len(anchor) - len(anchor.lstrip())]
    rows = [row.rstrip().rstrip(",") for row in lines[first:last + 1]]
    if new.rstrip().rstrip(",") in rows: raise SystemExit("exclusion already present: " + new.strip())
    rows.append(new.rstrip().rstrip(","))
    rows.sort(key=lambda row: row.strip().encode("utf-16-be"))
    block = [row + ("," if index < len(rows) - 1 else "") for index, row in enumerate(rows)]
    return "\n".join(lines[:first] + block + lines[last + 1:])


def core(base):
    """🧱️ The protocol moves to the framework runtime (`⚙️runtime/🗣️language`, below every module that renders texts), gains its notice rule and every rule as data; Python oracle + cross-runtime law."""
    out = {
        RT + "🔣️.json": src("protocol.json"),
        RT + "🧬️schema/🔣️.json": src("schema.json"),
        RT + "🟦️.ts": src("language.ts"),
        RT + "🧪️tests/🔣️.json": src("vectors.json"),
        RT + "🧪️tests/🟦️.ts": src("test.ts"),
        RT + "🧪️tests/🐍️.py": src("oracle.py"),
        ENTRY: ('/** 🗣️ Public `@semio-tech/framework/runtime/language` entry: explicit re-exports of the module that owns each member. */\n'
                'export { chooseLanguageV1, LANGUAGE_PROTOCOL_V1, languageFlagV1, languageRefusalV1, languageSubtagV1, noticeLanguagesV1, osLanguageListV1, resolveLanguageV1 } from "../../../../🔨️modules/⚙️runtime/🗣️language/🟦️.ts";\n'
                'export type { LanguageEnvironmentV1, LanguageRefusalV1, LanguageResolutionV1, OsLanguageListV1 } from "../../../../🔨️modules/⚙️runtime/🗣️language/🟦️.ts";\n').encode(),
    }
    for name in ("🔣️.json", "🧬️schema/🔣️.json", "🟦️.ts", "🧪️tests/🔣️.json", "🧪️tests/🟦️.ts"): out[OLD + name] = None
    rel = F + "📦️packages/🟦️typescript/package.json"
    out[rel] = replace(base(rel).decode(), '    "./runtime/identity": "./⚙️runtime/🏷️identity/🟦️.ts",\n', '    "./runtime/identity": "./⚙️runtime/🏷️identity/🟦️.ts",\n    "./runtime/language": "./⚙️runtime/🗣️language/🟦️.ts",\n').encode()
    out[PROCESS_ENTRY] = ('/** 🗣️ Public entry `@semio-tech/process/build/language`: the language protocol of the framework runtime. */\n'
                          'export { resolveLanguageV1 } from "' + depth_to_framework(PROCESS_ENTRY) + '📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";\n'
                          'export type { LanguageEnvironmentV1, OsLanguageListV1 } from "' + depth_to_framework(PROCESS_ENTRY) + '📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";\n').encode()
    for rel, old in ((OLD + "📣️progress/🟦️.ts", '"../🟦️.ts"'), (F + "🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts", '"../../🗣️language/🟦️.ts"'), (F + "🔨️modules/🏃️process/🏗️build/🧰️toolchains/🟦️.ts", '"../🗣️language/🟦️.ts"')):
        out[rel] = replace(base(rel).decode(), old, '"' + depth_to_framework(rel) + '📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts"').encode()
    rel = LIB.replace("📚️library/", "") + "🚀️launcher/🗂️registry/🔣️.json"
    out[rel] = replace(base(rel).decode(), '"' + OLD + '🔣️.json"', '"' + RT + '🔣️.json"').encode()
    rel = LIB.replace("📚️library/", "") + "🚀️launcher/🧪️tests/🗣️language-contract/🐹️.go"
    out[rel] = replace(base(rel).decode(), '"' + OLD + '🧪️tests/🔣️.json"', '"' + RT + '🧪️tests/🔣️.json"').encode()
    rel = LIB + "🏗️build/🔧️tools/🏗️bazel/🎬️scopes/🔣️.json"
    out[rel] = scopes_edit(base(rel).decode()).encode()
    rel = LIB + "🏗️build/🏃️execution/🔒️hermetic/🔣️.json"
    text = replace(base(rel).decode(), '"' + OLD + '🟦️.ts"', '"' + RT + '🟦️.ts"')
    out[rel] = replace(text, '"' + OLD + '🔣️.json"', '"' + RT + '🔣️.json"').encode()
    for rel, count in (("nx.json", 1), (LIB + "🏗️build/🔣️policy.json", 1), (LIB + "🏗️build/📥️inputs/🎯️precision/🔣️.json", 3), (LIB.replace("📚️library/", "") + "🎛️dashboard/📦️packages/🦀️rust/📋️project.json", 1)):
        text = base(rel).decode()
        for line in sorted({l for l in text.split("\n") if ("{workspaceRoot}/" + OLD + "**/*") in l}):
            if line.lstrip().startswith('"!{workspaceRoot}/'): text = insert_sorted(text, line, line.replace(OLD, RT))
            else: text = replace(text, line + "\n", line + "\n" + line.replace(OLD, RT) + "\n", text.count(line + "\n"))
        if text.count("{workspaceRoot}/" + RT) != count: raise SystemExit(f"{rel}: runtime glob count {text.count('{workspaceRoot}/' + RT)}")
        out[rel] = text.encode()
    rel = LIB + "🏗️build/📥️source/🔏️closure/🗓️scheduling/🔣️.json"
    out[rel] = replace(base(rel).decode(), '        "' + OLD + '",\n', '        "' + OLD + '",\n        "' + RT + '",\n').encode()
    rel = LIB + "⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🔏️admission/🧫️.json"
    text = replace(base(rel).decode(), '"' + OLD + '🟦️.ts"', '"' + RT + '🟦️.ts"')
    out[rel] = replace(text, '"' + OLD + '🧬️schema/🔣️.json"', '"' + RT + '🧬️schema/🔣️.json"').encode()
    return out


def entry_of(rel):
    return "../🗣️language/🟦️.ts" if rel.startswith(F + "🔨️modules/⚙️runtime/") else depth_to_framework(rel) + "📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts"


def add_import(text, rel, names):
    line = ("import {" + ",".join(n for n in names if not (rel.endswith(".mjs") and n.startswith("type "))) + '} from "' + entry_of(rel) + '";\n')
    at = text.find("\nimport") + 1 if not text.startswith("import") else 0
    return text[:at] + line + text[at:]


def block(text, signature):
    """🧱️ The span of the top-level function that starts with `signature`, with its one-line docstring before it."""
    start = text.find(signature)
    if start < 0 or text.count(signature) != 1: raise SystemExit(f"signature found {text.count(signature)}x: {signature[:90]!r}")
    doc = text.rfind("\n", 0, start - 1) + 1
    if text[doc:start].lstrip().startswith("/**"): start = doc
    open_at, depth, index = text.index("{", text.index(")", text.find(signature)) ), 0, None
    for index in range(open_at, len(text)):
        char = text[index]
        if char == "\\": continue
        if text[index - 1] == "\\": continue
        depth += char == "{"
        depth -= char == "}"
        if depth == 0: break
    end = index + 1
    if text[end:end + 1] == "\n": end += 1
    return start, end


def swap(text, signature, new):
    start, end = block(text, signature)
    return text[:start] + new.rstrip("\n") + "\n" + text[end:]


def drop_json_key(text, key, indent):
    data = json.loads(text)
    def walk(node):
        if isinstance(node, dict):
            node.pop(key, None)
            if isinstance(node.get("required"), list) and key in node["required"]: node["required"].remove(key)
            for value in node.values(): walk(value)
        elif isinstance(node, list):
            for value in node: walk(value)
    walk(data)
    return json.dumps(data, ensure_ascii=False, indent=indent, separators=(",", ": ")) + "\n"


def copies(base):
    """🧹️ Every language picker copy (16 in TypeScript/JavaScript) becomes a call of the one protocol: a command resolves (and refuses), a notice is written in the chosen language or in every offered one; the retired variables SEMIO_DASHBOARD_LOCALE and SEMIO_LOCALE and the schema copies of the variable order (`languageEnvironment`, `systemVariables`, `fallback`) are gone."""
    out, P_, R_ = {}, F + "🔨️modules/🏃️process/", LIB
    def edit(rel, fn):
        out[rel] = fn(out[rel].decode() if rel in out else base(rel).decode()).encode()
    NOTICE = ["noticeLanguagesV1", "type OsLanguageListV1"]

    rel = F + "🔨️modules/⚙️runtime/🗣️messages/🟦️.ts"
    def runtime(text):
        start, end = block(text, "function messageLanguage(")
        text = text[:start] + text[end:].lstrip("\n")
        text = replace(text, "/** 🗣️ Renders one catalog message in the language the environment names, or in every language when it names none; `{detail}` may name another catalog message. */\nexport function runtimeMessageV1(id: string, parameters: Readonly<Record<string, string | number>>): string {",
                       "/** 🗣️ Renders one catalog message in the language the language protocol chooses, or in every language when it chooses none; `{detail}` may name another catalog message. */\nexport function runtimeMessageV1(id: string, parameters: Readonly<Record<string, string | number>>, languages: readonly (\"en\" | \"de\")[] = noticeLanguagesV1([\"en\", \"de\"] as const)): string {")
        text = replace(text, "  const language = messageLanguage();\n  return language === undefined ? `${render(\"en\", row)} / ${render(\"de\", row)}` : render(language, row);", "  return languages.map((language) => render(language, row)).join(\" / \");")
        return add_import(text, rel, ["noticeLanguagesV1"])
    edit(rel, runtime)
    rel = F + "🔨️modules/⚙️runtime/🧪️tests/🟦️.ts"
    def runtime_test(text):
        start = text.index("    const saved = { ...process.env };\n    try {\n      for (const name of [\"SEMIO_LANGUAGE\"")
        end = text.index("\n", text.index("    } finally { for (const name of [\"SEMIO_LANGUAGE\"", start)) + 1
        new = ('    const notice = (environment: Record<string, string>) => noticeLanguagesV1(["en", "de"] as const, environment);\n'
               '    expect(runtimeMessageV1("bodyTooLarge", {}, notice({}))).toBe("The request body is too large. / Der Anfragekörper ist zu groß.");\n'
               '    expect(runtimeMessageV1("moduleNotFound", { specifier: "x", from: "y" }, notice({ SEMIO_LANGUAGE: "de" }))).toBe("Das Modul \'x\' ist von \'y\' aus nicht auffindbar.");\n'
               '    expect(runtimeMessageV1("syntax", { format: "TOML", detail: "toml.keyDefined", name: "a", position: "1:2" }, notice({ SEMIO_LANGUAGE: "en" }))).toBe("TOML syntax error: key a is already defined at 1:2");\n'
               '    expect(runtimeMessageV1("commandRequired", {}, notice({ SEMIO_LANGUAGE: "fr", LANG: "de_CH.UTF-8" }))).toBe(`${(messages as Record<string, { en: string; de: string }>).commandRequired!.en} / Ein Befehl ist erforderlich.`);\n'
               '    expect(runtimeMessageV1("commandRequired", {}, notice({ LANG: "de_CH.UTF-8" }))).toBe("Ein Befehl ist erforderlich.");\n')
        text = text[:start] + new + text[end:]
        return add_import(text, rel, ["noticeLanguagesV1"])
    edit(rel, runtime_test)

    rel = P_ + "📦️artifacts/🏗️native-build/💾️storage/🧰️programs/🟦️.ts"
    def programs(text):
        start = text.index("const language=():\"en\"|\"de\"|null=>{")
        text = text[:start] + text[text.index("\n", start) + 1:]
        text = replace(text, ',only=language();return only?fill(row[only]):fill(row.en)+" / "+fill(row.de);};', ';return noticeLanguagesV1(["en","de"] as const).map(language=>fill(row[language])).join(" / ");};')
        return add_import(text, rel, ["noticeLanguagesV1"])
    edit(rel, programs)

    rel = P_ + "🧪️testing/🧰️toolchain/🟦️.ts"
    edit(rel, lambda text: add_import(swap(text, "export function describeMissingTestToolV1(", '/** 🗣️ Renders the missing-tool message in the language the language protocol chooses, one line per declared language when it chooses none. */\nexport function describeMissingTestToolV1(resolution:TestToolResolutionV1,environment:Readonly<Record<string,string|undefined>>=process.env):string{\n return noticeLanguagesV1(protocol.languages as readonly ("en"|"de")[],environment).map(language=>protocol.messages.missing[language].replaceAll("{tool}",resolution.tool).replaceAll("{tried}",resolution.tried.join(", "))).join("\\n");\n}'), rel, ["noticeLanguagesV1"]))

    rel = P_ + "🔒️leases/📈️standing/🤝️yield/🟦️.ts"
    edit(rel, lambda text: add_import(swap(text, "export function leaseLanguagesV1(", '/** 🌐️ The languages a notice is written in: the one the language protocol chooses, both when it chooses none. */\nexport function leaseLanguagesV1(environment:Readonly<Record<string,string|undefined>>=process.env):readonly ("en"|"de")[]{\n return noticeLanguagesV1(["en","de"] as const,environment);\n}'), rel, ["noticeLanguagesV1"]))

    rel = P_ + "🔒️leases/🧭️order/🟦️.ts"
    def order(text):
        start, end = block(text, "export function leaseLanguageV1(")
        text = text[:start] + text[end:].lstrip("\n")
        text = replace(text, "const recorded=Object.freeze({...row,enforced:rule===\"enforce\"}),language=leaseLanguageV1(),message=formatLeaseViolationV1(language,row);", "const recorded=Object.freeze({...row,enforced:rule===\"enforce\"}),languages=noticeLanguagesV1([\"en\",\"de\"] as const),message=languages.map(language=>formatLeaseViolationV1(language,row)).join(\"\\n\");")
        text = replace(text, "process.stderr.write(fill(schema.definitions.Messages.const.observed[language],{message})+\"\\n\");", "process.stderr.write(languages.map(language=>fill(schema.definitions.Messages.const.observed[language],{message:formatLeaseViolationV1(language,row)})).join(\"\\n\")+\"\\n\");")
        return add_import(text, rel, ["noticeLanguagesV1"])
    edit(rel, order)
    rel = P_ + "📦️artifacts/🏗️native-build/🔒️lease/🔐️custody/🟦️.ts"
    def custody(text):
        text = replace(text, 'import { leaseLanguageV1 } from "../../../../🔒️leases/🧭️order/🟦️.ts";\n', "")
        text = replace(text, 'process.stderr.write(messages.waiting[leaseLanguageV1()].replace("{directory}", buildDirectory).replace("{profile}", profile).replace("{pid}", String(holder ?? "?")).replace("{seconds}", String(seconds)) + "\\n");', 'process.stderr.write(noticeLanguagesV1(["en", "de"] as const).map((language) => messages.waiting[language].replace("{directory}", buildDirectory).replace("{profile}", profile).replace("{pid}", String(holder ?? "?")).replace("{seconds}", String(seconds))).join("\\n") + "\\n");')
        return add_import(text, rel, ["noticeLanguagesV1"])
    edit(rel, custody)
    rel = P_ + "🔒️leases/📋️status/🟦️.ts"
    def status(text):
        text = replace(text, "leaseLanguageV1,", "")
        text = replace(text, "language=leaseLanguageV1(context.environment)", "language=resolveLanguageV1([\"en\",\"de\"] as const,args,context.environment).language")
        return add_import(text, rel, ["resolveLanguageV1"])
    edit(rel, status)
    rel = R_ + "⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts"
    def opening(text):
        text = replace(text, 'import {leaseLanguageV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🧭️order/🟦️.ts";\n', "")
        text = replace(text, "const language=leaseLanguageV1();", "const languages=noticeLanguagesV1([\"en\",\"de\"] as const);")
        text = replace(text, "for(const line of formatResourceLeaseSweepV1(language,sweep))process.stderr.write(sweepNoticeV1(line));", "for(const language of languages)for(const line of formatResourceLeaseSweepV1(language,sweep))process.stderr.write(sweepNoticeV1(line));")
        text = replace(text, "catch(error){process.stderr.write(sweepNoticeV1(formatResourceLeaseSweepFailureV1(language,error)));}", "catch(error){for(const language of languages)process.stderr.write(sweepNoticeV1(formatResourceLeaseSweepFailureV1(language,error)));}")
        return add_import(text, rel, ["noticeLanguagesV1"])
    edit(rel, opening)
    rel = R_ + "🏗️build/🚢️fleet-status/⌨️command/🟦️.ts"
    def fleet(text):
        text = replace(text, 'import {leaseLanguageV1} from "../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🟦️.ts";\n', "")
        text = replace(text, "language:FleetLanguageV1=leaseLanguageV1(environment)", "language:FleetLanguageV1=resolveLanguageV1([\"en\",\"de\"] as const,args,environment).language")
        return add_import(text, rel, ["resolveLanguageV1"])
    edit(rel, fleet)

    rel = F + "🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts"
    edit(rel, lambda text: add_import(swap(text, "export function churnLanguagesV1(", "/** 🌐️ The languages a message is rendered in: the one the language protocol chooses, every declared language when it chooses none. */\nexport function churnLanguagesV1(environment:Readonly<Record<string,string|undefined>>=process.env):readonly string[]{\n return noticeLanguagesV1(protocol.languages,environment);\n}"), rel, ["noticeLanguagesV1"]))

    rel = R_ + "🏗️build/🏛️sealed/🟦️.ts"
    edit(rel, lambda text: add_import(swap(text, "export function sealedLanguagesV1(", "/** 🌐️ The languages a message is rendered in: the one the language protocol chooses, both declared languages when it chooses none. */\nexport function sealedLanguagesV1(environment:Readonly<Record<string,string|undefined>>=process.env,policy:SealedPolicyV1=sealedPolicyV1):readonly SealedLanguageV1[]{\n return noticeLanguagesV1(policy.languages,environment);\n}"), rel, ["noticeLanguagesV1"]))
    rel = R_ + "🏗️build/🧷️format-stamps/🟨️.mjs"
    edit(rel, lambda text: add_import(swap(text, "export function formatStampLanguagesV1(", "/** 🌐️ The languages a message is rendered in: the one the language protocol chooses, both declared languages when it chooses none. */\nexport function formatStampLanguagesV1(environment=process.env){\n return [...noticeLanguagesV1(protocol.languages,environment)];\n}"), rel, ["noticeLanguagesV1"]))
    rel = R_ + "🎮️playground/🖥️native-host/🟨️.mjs"
    edit(rel, lambda text: add_import(swap(text, "export function nativeHostLanguagesV1(", "/** 🌐️ The languages the refusal is rendered in: the one the language protocol chooses, both declared languages when it chooses none. */\nexport function nativeHostLanguagesV1(environment=process.env) {\n  return [...noticeLanguagesV1(messages.languages,environment)];\n}"), rel, ["noticeLanguagesV1"]))
    for rel, sig, doc in ((R_ + "🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🏊️pool/🟨️.mjs", "export function selectGraphFarmLanguagesV1(", "export function selectGraphFarmLanguagesV1(environment) {\n  return [...noticeLanguagesV1(P.languages, environment)];\n}"),
                          (R_ + "🏗️build/🧠️memory/🟨️.mjs", "export function selectGraphMemoryLanguagesV1(", "/** 🌐️ The languages of the process: the one the language protocol chooses, else every supported language in declared order (English first, German second); there is no default language. */\nexport function selectGraphMemoryLanguagesV1(environment){\n return [...noticeLanguagesV1(P.languages,environment)];\n}")):
        edit(rel, lambda text, sig=sig, doc=doc, rel=rel: add_import(swap(text, sig, doc), rel, ["noticeLanguagesV1"]))
    for rel in (R_ + "🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🏊️pool/🧬️schema/🔣️.json",):
        edit(rel, lambda text: replace(text, '        "languageEnvironment": ["SEMIO_LANGUAGE", "SEMIO_DASHBOARD_LOCALE", "LC_ALL", "LC_MESSAGES", "LANG"],\n', ""))
    edit(R_ + "🏗️build/🧠️memory/🧬️schema/🔣️.json", lambda text: drop_json_key(text, "languageEnvironment", 1))
    edit(R_ + "🏗️build/🧠️memory/🧪️tests/🔣️.json", lambda text: drop_json_key(text, "languageEnvironment", 1))
    for rel in (R_ + "🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🏊️pool/🟨️.d.mts", R_ + "🏗️build/🧠️memory/🟨️.d.mts"):
        edit(rel, lambda text: replace(text, "readonly languageEnvironment:readonly string[];", ""))

    rel = R_ + "🏗️build/🌐️remote-cache/⌨️command/🟦️.ts"
    edit(rel, lambda text: add_import(replace(replace(text, " for(const value of[environment.SEMIO_LANGUAGE,environment.LC_ALL,environment.LC_MESSAGES,environment.LANG]){const head=value?.slice(0,2).toLowerCase();if(head&&LANGUAGES.includes(head as RemoteCacheLanguageV1))return [head as RemoteCacheLanguageV1];}\n return LANGUAGES;\n", " return noticeLanguagesV1(LANGUAGES,environment);\n"), "/** ☁️ Picks the output languages: the explicit request, else the one the environment names (`SEMIO_LANGUAGE`, then the locale), else all of them, because no language is the default. */", "/** ☁️ Picks the output languages: the explicit request, else the one the language protocol chooses, else all of them, because no language is the default. */"), rel, ["noticeLanguagesV1"]))
    rel = R_ + "🏗️build/🩺️doctor/🟦️.ts"
    def doctor(text):
        text = replace(text, " for(const value of [environment.SEMIO_LANGUAGE,environment.LANGUAGE,environment.LC_ALL,environment.LC_MESSAGES,environment.LANG]){const chosen=tag(value);if(chosen)return [chosen];}\n return [\"en\",\"de\"];\n", " return noticeLanguagesV1([\"en\",\"de\"] as const,environment);\n")
        text = replace(text, ' const tag=(value:string|undefined):DoctorLanguageV1|undefined=>{const head=value?.toLowerCase().split(/[._@:-]/u)[0];return head==="en"||head==="de"?head:undefined;};\n if(requested!==undefined){const chosen=tag(requested);if(!chosen||requested.length>2)throw', ' if(requested!==undefined){const chosen=requested==="en"||requested==="de"?requested:undefined;if(!chosen)throw')
        text = replace(text, "/** 🗣️ Chooses the output languages: the request, then the environment, then the locale; when none resolves both languages are printed, because no language is assumed. */", "/** 🗣️ Chooses the output languages: the request, else the language protocol; when it chooses none both languages are printed, because no language is assumed. */")
        return add_import(text, rel, ["noticeLanguagesV1"])
    edit(rel, doctor)
    rel = R_ + "⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🟦️.ts"
    def sites(text):
        text = replace(text, "const tag=[environment.SEMIO_LANGUAGE,environment.LC_ALL,environment.LC_MESSAGES,environment.LANG].map(value=>value?.trim().toLowerCase().split(/[._@-]/u)[0]).find(value=>value&&row[value]),languages=tag?[tag]:Object.keys(row),", "const tag=environment.SEMIO_LANGUAGE,languages=tag&&row[tag]?[tag]:Object.keys(row),")
        text = replace(text, "/** 🗣️ Renders one refusal in the requested language, or in every language when none is resolved; self-contained so loaders can embed its source. */", "/** 🗣️ Renders one refusal in the language the door exported as `SEMIO_LANGUAGE`, or in every language when none is exported; self-contained (no import) so loaders can embed its source. */")
        text = replace(text, "environment:Readonly<Record<string,string|undefined>>=process.env):Error{return Error(closureRefusalTextV1(messages,code,fields,environment));}", "environment:Readonly<Record<string,string|undefined>>=process.env):Error{const languages=noticeLanguagesV1(Object.keys(messages[code]),environment);return Error(closureRefusalTextV1(messages,code,fields,{SEMIO_LANGUAGE:languages.length===1?languages[0]:undefined}));}")
        return add_import(text, rel, ["noticeLanguagesV1"])
    edit(rel, sites)

    rel = R_.replace("📚️library/", "") + "💻️client/🛠️developer-tools/🏃️process/🟦️.ts"
    edit(rel, lambda text: add_import(replace(text, 'export function developerMessageV1(id:keyof typeof messages):string {const language=process.env.SEMIO_LOCALE;return language==="en"||language==="de"?messages[id][language]:`${messages[id].en} / ${messages[id].de}`;}', 'export function developerMessageV1(id:keyof typeof messages):string {return noticeLanguagesV1(["en","de"] as const).map(language=>messages[id][language]).join(" / ");}'), rel, ["noticeLanguagesV1"]))
    loader = src("oracle-loader.py").decode()
    rel = R_ + "🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🐍️.py"
    edit(rel, lambda text: replace(replace(text, src("farm-old.py").decode(), src("farm-new.py").decode()), "\n\n\ndef plan(projects, environment, cores):", "\n\n\n" + loader + "\n\ndef plan(projects, environment, cores):"))
    rel = R_ + "🏗️build/🧠️memory/🧪️tests/🐍️.py"
    edit(rel, lambda text: replace(replace(text, src("memory-old.py").decode(), src("memory-new.py").decode()), "\n\n\ndef decide(policy, state):", "\n\n\n" + loader + "\n\ndef decide(policy, state):"))
    rel = R_ + "🏗️build/🧠️memory/🧪️tests/🔣️.json"
    def memory_rows(text):
        data = json.loads(text)
        rows = {row["name"]: row for row in data["languages"]}
        data["languages"].remove(rows["dashboard-locale-before-system-locale"])
        for row in [*data["languages"], *data["messageCases"]]: row["environment"].pop("SEMIO_DASHBOARD_LOCALE", None)
        rows["unsupported-locale-falls-through"].update({"name": "unsupported-lc-all-hides-lang", "expect": ["en", "de"]})
        data["languages"].append({"name": "preference-list-before-locale", "environment": {"LANGUAGE": "fr:de", "LANG": "en_US.UTF-8"}, "expect": ["de"]})
        return json.dumps(data, ensure_ascii=False, indent=1, separators=(",", ": ")) + "\n"
    edit(rel, memory_rows)
    rel = F + "🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/🐍️.py"
    edit(rel, lambda text: replace(text, src("churn-old.py").decode(), loader + "\n\n" + src("churn-new.py").decode()))
    rel = R_ + "🏗️build/🚪️door/🟦️.ts"
    edit(rel, lambda text: add_import(replace(text, 'tag=environment.SEMIO_LANGUAGE?.trim().toLowerCase().split(/[._@:-]/u)[0],languages:readonly ("en"|"de")[]=tag==="en"||tag==="de"?[tag]:["en","de"],', 'languages:readonly ("en"|"de")[]=noticeLanguagesV1(["en","de"] as const,environment),'), rel, ["noticeLanguagesV1"]))
    rel = R_.replace("📚️library/", "") + "💻️client/🛠️developer-tools/📜️script.ts"
    edit(rel, lambda text: add_import(replace(text, 'const language=process.env.SEMIO_LOCALE,title=language==="en"||language==="de"?row.title[language]:`${row.title.en} / ${row.title.de}`;', 'const title=noticeLanguagesV1(["en","de"] as const).map(language=>row.title[language]).join(" / ");'), rel, ["noticeLanguagesV1"]))
    rel = R_ + "🏗️build/🏛️sealed/🧪️tests/🗂️store/🟦️.ts"
    def sealed_test(text):
        text = replace(text, "const original=readFileSync(fileURLToPath(new URL(\"../../🟦️.ts\",import.meta.url)),\"utf8\").replace('\"./🔣️.json\"','\"./policy.json\"'),core=", "const original=readFileSync(fileURLToPath(new URL(\"../../🟦️.ts\",import.meta.url)),\"utf8\").replace('\"./🔣️.json\"','\"./policy.json\"').replace('\"" + entry_of(R_ + "🏗️build/🏛️sealed/🟦️.ts") + "\"','\"./language.ts\"'),core=")
        module = depth_to_framework(rel) + "🔨️modules/⚙️runtime/🗣️language/"
        return replace(text, "writeFileSync(join(w.root,\"tool/policy.json\"),JSON.stringify(policyData));", "writeFileSync(join(w.root,\"tool/policy.json\"),JSON.stringify(policyData));writeFileSync(join(w.root,\"tool/language.ts\"),readFileSync(fileURLToPath(new URL(\"" + module + "🟦️.ts\",import.meta.url)),\"utf8\").replace('\"./🔣️.json\"','\"./language.json\"'));writeFileSync(join(w.root,\"tool/language.json\"),readFileSync(fileURLToPath(new URL(\"" + module + "🔣️.json\",import.meta.url)),\"utf8\"));")
    edit(rel, sealed_test)
    rel = R_ + "🏗️build/🧷️format-stamps/🧪️tests/🟦️.ts"
    edit(rel, lambda text: replace(text, 'expect(formatStampLanguagesV1({SEMIO_LANGUAGE:"fr",LANG:"de_CH.UTF-8"})).toEqual(["de"]);', 'expect(formatStampLanguagesV1({SEMIO_LANGUAGE:"fr",LANG:"de_CH.UTF-8"}),"an unoffered explicit choice chooses nothing").toEqual(["en","de"]);'))
    rel = F + "🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/🟦️.ts"
    def churn_test(text):
        text = replace(text, 'test("an input that is rewritten on every read fails after the declared attempts, naming the path in both languages",async()=>{', 'test("an input that is rewritten on every read fails after the declared attempts, naming the path in the chosen language",async()=>{')
        text = replace(text, " const failure=await reader.digest(path).catch(error=>error);\n expect(failure).toBeInstanceOf(ConcurrentEditErrorV1);\n expect(failure.path).toBe(path);expect(failure.attempts).toBe(protocol.profiles.read.maximumAttempts);\n expect(failure.message).toContain(path);expect(failure.message).toContain(\"kept changing\");expect(failure.message).toContain(\"änderte sich\");",
                       " const saved=process.env.SEMIO_LANGUAGE;process.env.SEMIO_LANGUAGE=\"de\";\n const failure=await reader.digest(path).catch(error=>error).finally(()=>{if(saved===undefined)delete process.env.SEMIO_LANGUAGE;else process.env.SEMIO_LANGUAGE=saved;});\n expect(failure).toBeInstanceOf(ConcurrentEditErrorV1);\n expect(failure.path).toBe(path);expect(failure.attempts).toBe(protocol.profiles.read.maximumAttempts);\n expect(failure.message).toContain(path);expect(failure.message).toContain(\"änderte sich\");expect(failure.message).not.toContain(\"kept changing\");")
        return text
    edit(rel, churn_test)
    return out


def grep_files(needle, pathspec, walk_root):
    """🔍️ The tracked files that contain `needle` (`git grep` in a checkout; a mirror has no `.git`, so its files under `walk_root` are walked)."""
    import subprocess
    if os.path.exists(os.path.join(ROOT, ".git")):
        return subprocess.run(["git", "grep", "-l", "-F", needle, "--", pathspec], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")
    found = []
    for folder, names, files in os.walk(os.path.join(ROOT, walk_root)):
        names[:] = [n for n in names if n not in ("node_modules", "target", "dist") and not n.startswith(".")]
        for name in files:
            path = os.path.join(folder, name)
            if os.path.getsize(path) < (4 << 20) and needle in open(path, encoding="utf8", errors="replace").read(): found.append(os.path.relpath(path, ROOT))
    return found


def imports(base):
    """🔗️ Every importer of `@semio-tech/process/build/language` imports `@semio-tech/framework/runtime/language` directly; the forwarding entry and its export are gone."""
    import subprocess
    old, new = "🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🟦️.ts", "📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts"
    listed = grep_files(old, ":!.🧬semio", "🧰️framework")
    out = {}
    for rel in filter(None, listed):
        text = base(rel)
        if text is None or rel == PROCESS_ENTRY: continue
        out[rel] = text.decode().replace(old, new).encode()
    if len(out) < 30: raise SystemExit(f"imports: only {len(out)} importers found")
    out[PROCESS_ENTRY] = None
    rel = F + "🔨️modules/🏃️process/📦️packages/🟦️typescript/package.json"
    out[rel] = replace(base(rel).decode(), '    "./build/language": "./🏗️build/🗣️language/🟦️.ts",\n', "").encode()
    return out


def gofmt(text):
    import subprocess
    go = os.environ.get("LP_GO", "go")
    done = subprocess.run([os.path.join(os.path.dirname(go), "gofmt") if os.path.dirname(go) else "gofmt"], input=text, capture_output=True, text=True)
    if done.returncode != 0: raise SystemExit("gofmt: " + done.stderr[:400])
    return done.stdout


def launcher(base):
    """🐹️ The Go launcher reads every rule of the one protocol (flag, `--` end, separators, macOS command and parse pattern, refusal texts per kind) instead of hard-coding them; its three registry copies of the refusal texts are no longer required (they are dropped once the rebuilt launcher is installed: batch `launcher2`)."""
    out, G = {}, LIB.replace("📚️library/", "") + "🚀️launcher/📦️packages/🐹️go/"
    rel = G + "🗣️language/🐹️.go"
    text = base(rel).decode()
    start, end = text.index("// 📖️LanguageProtocol is"), text.index("// 🌍️EverySay renders")
    out[rel] = gofmt(text[:start] + src("language-region.go").decode() + text[end:]).encode()
    rel = G + "🐚️unix-host/🐹️.go"
    text = replace(base(rel).decode(), 'var appleLanguage = regexp.MustCompile(`"?([A-Za-z]{2,3}(?:-[A-Za-z0-9]+)*)"?\\s*[,)]`)\n\n', "")
    text = replace(text, "// 🖥️systemLanguages lists the macOS preferred languages; other Unix systems state theirs in the environment.\nfunc systemLanguages() []string {", "// 🖥️systemLanguages lists the macOS preferred languages by the command and pattern of the language protocol; other Unix systems state theirs in the environment.\nfunc systemLanguages(locale OsLocale) []string {")
    text = replace(text, '	deadline, stop := context.WithTimeout(context.Background(), 3*time.Second)\n	defer stop()\n	output, err := exec.CommandContext(deadline, "defaults", "read", "-g", "AppleLanguages").Output()\n	if err != nil {\n		return nil\n	}\n	var tags []string\n	for _, match := range appleLanguage.FindAllStringSubmatch(string(output), -1) {',
                   '	pattern, err := regexp.Compile(locale.Darwin.Pattern)\n	if err != nil || locale.Darwin.Command == "" {\n		return nil\n	}\n	deadline, stop := context.WithTimeout(context.Background(), time.Duration(locale.Darwin.TimeoutMs)*time.Millisecond)\n	defer stop()\n	output, err := exec.CommandContext(deadline, locale.Darwin.Command, locale.Darwin.Args...).Output()\n	if err != nil {\n		return nil\n	}\n	var tags []string\n	for _, match := range pattern.FindAllStringSubmatch(string(output), -1) {')
    out[rel] = gofmt(text).encode()
    rel = G + "🪟️windows-host/🐹️.go"
    out[rel] = replace(base(rel).decode(), "func systemLanguages() []string {", "func systemLanguages(OsLocale) []string {").encode()
    rel = G + "🏗️entry/🐹️.go"
    text = replace(base(rel).decode(), "	System  func() []string\n", "	System  func(OsLocale) []string\n")
    out[rel] = replace(text, "		s.host.Stderr(s.registry.Refusal(refusal))\n", "		s.host.Stderr(protocol.Refuse(s.registry.Languages, refusal))\n").encode()
    rel = G + "🗂️runtime-registry/🐹️.go"
    out[rel] = replace(base(rel).decode(), '"languageRefusal", "languageUnknownFlag", "languageUnknownVariable", ', "").encode()
    T = LIB.replace("📚️library/", "") + "🚀️launcher/🧪️tests/"
    rel = T + "🧰️harness/🐹️.go"
    out[rel] = replace(base(rel).decode(), "		System:  func() []string { return nil },\n", "		System:  func(OsLocale) []string { return nil },\n").encode()
    rel = T + "🗣️language-contract/🐹️.go"
    text = replace(base(rel).decode(), "		got, refusal := protocol.ResolveLanguage(fixture.Languages, row.Args, environment(row.Environment), func() []string { return row.Os })\n", "		got, refusal := protocol.ResolveLanguage(fixture.Languages, row.Args, environment(row.Environment), func(OsLocale) []string { return row.Os })\n")
    text = replace(text, "		text := strings.Split(registry.Refusal(refusal), \"\\n\")\n", "		text := strings.Split(protocol.Refuse(registry.Languages, refusal), \"\\n\")\n")
    start, end = text.index("// 🗣️The flag reader stops at the separator"), text.index("// #endregion 🗣️Language")
    text = text[:start] + ('// 🗣️The flag reader stops at the separator and keeps the last value.\nfunc TestLanguageFlag(t *testing.T) {\n\tprotocol, err := LoadLanguageProtocol(testRoot(t), loadRegistry(t))\n\tif err != nil {\n\t\tt.Fatal(err)\n\t}\n'
                           '\tif value, given := protocol.LanguageFlag([]string{"a", "--lang", "de", "--", "--lang=en"}); !given || value != "de" {\n\t\tt.Fatal(value)\n\t}\n'
                           '\tif value, _ := protocol.LanguageFlag([]string{"--lang=en", "--lang=de"}); value != "de" {\n\t\tt.Fatal(value)\n\t}\n'
                           '\tif _, given := protocol.LanguageFlag(nil); given {\n\t\tt.Fatal("no flag")\n\t}\n}\n\n'
                           '// 🍎️The macOS command output is parsed by the pattern of the protocol.\nfunc TestLanguageSystemPattern(t *testing.T) {\n\tprotocol, err := LoadLanguageProtocol(testRoot(t), loadRegistry(t))\n\tif err != nil {\n\t\tt.Fatal(err)\n\t}\n'
                           '\tpattern := regexp.MustCompile(protocol.OsLocale.Darwin.Pattern)\n\tvar tags []string\n\tfor _, match := range pattern.FindAllStringSubmatch("(\\n    \\"en-US\\",\\n    \\"de-CH\\",\\n    \\"zh-Hans-CN\\"\\n)", -1) {\n\t\ttags = append(tags, match[1])\n\t}\n'
                           '\tif strings.Join(tags, " ") != "en-US de-CH zh-Hans-CN" {\n\t\tt.Fatal(tags)\n\t}\n}\n\n') + text[end:]
    text = replace(text, 'import (\n\t"strings"\n\t"testing"\n)', 'import (\n\t"regexp"\n\t"strings"\n\t"testing"\n)')
    out[rel] = gofmt(text).encode()
    for rel in [r for r in (G + "🛰️resident/🐹️.go",)]:
        out[rel] = base(rel)
    return out


def launcher2(base):
    """🧽️ The registry no longer carries its copies of the language refusal texts (`languageRefusal`, `languageUnknownFlag`, `languageUnknownVariable`); land only after the launcher of batch `launcher` is rebuilt and installed, because an older launcher still requires them."""
    rel = LIB.replace("📚️library/", "") + "🚀️launcher/🗂️registry/🔣️.json"
    text = base(rel).decode()
    for key in ("languageRefusal", "languageUnknownFlag", "languageUnknownVariable"):
        start = text.index('\n  "' + key + '": {\n') + 1
        end = text.index("\n  }", start) + len("\n  }")
        text = text[:start] + text[end:].lstrip(",\n")
    text = text.replace("},\n }\n}", "}\n }\n}")
    json.loads(text)
    return {rel: text.encode()}


def dashboard(base):
    """🎛️ The Rust dashboard reads the one protocol (`include_str!` of `⚙️runtime/🗣️language/🔣️.json`): SEMIO_LANGUAGE (which the repo door exports with its choice), the preference list and the first set locale variable decide, otherwise the dashboard asks; its own copy of the variable order (`systemVariables`) and the retired SEMIO_LOCALE are gone; its law resolves the shared protocol vectors."""
    out, D = {}, LIB.replace("📚️library/", "") + "🎛️dashboard/"
    def edit(rel, fn):
        out[rel] = fn(out[rel].decode() if rel in out else base(rel).decode()).encode()
    out[D + "⚙️preferences/🌐️language/🦀️.rs"] = src("dashboard-language.rs")
    out[D + "⚙️preferences/🌐️language/🧪️tests/🔬️unit/🦀️.rs"] = src("dashboard-language-test.rs")
    edit(D + "⚙️preferences/🌐️language/🔣️.json", lambda text: replace(text, '  "systemVariables": [\n    "LC_ALL",\n    "LC_MESSAGES",\n    "LANG"\n  ],\n', ""))
    edit(D + "🧬️schema/⚙️preferences/🌐️language/🔣️.json", lambda text: drop_json_key(text, "systemVariables", 2).replace('"description": "Supported system or explicit choices resolve the language.', '"description": "The language protocol of the repository (explicit variable, preference list, locale variables) or an explicit choice resolves the language.'))
    def fixture(text):
        data = json.loads(text)
        data.pop("resolve")
        for row in data["choiceOrders"]: row["declaration"].pop("systemVariables", None)
        rows = data["invalidDeclarations"]
        data["invalidDeclarations"] = [row for row in rows if row.get("systemVariables") != ["LANG", "LANG"]]
        for row in data["invalidDeclarations"]:
            if row.get("systemVariables") != ["lang"]: row.pop("systemVariables", None)
        return json.dumps(data, ensure_ascii=False, indent=2) + "\n"
    edit(D + "🧪️tests/🧫️fixtures/🌐️language/🔣️.json", fixture)
    edit(D + "⚙️preferences/🦀️.rs", lambda text: replace(text, '[("language", "SEMIO_LOCALE"), ', '[("language", language::protocol().explicit_variable()), '))
    def ts_test(text):
        start = text.index("const resolve = (chain: { systemVariables: string[];")
        end = text.index("};\n", start) + 3
        text = text[:start] + src("dashboard-resolver.ts").decode() + text[end:]
        return replace(text, "  for (const row of vectors.resolve) expect(resolve(declaration, row.environment), JSON.stringify(row)).toBe(row.expected);\n  for (const row of vectors.choiceOrders) expect(resolve(row.declaration, {}), JSON.stringify(row)).toBe(row.expected);",
                       "  const rows = shared.resolve.filter((row: { args: string[]; os: string[] }) => row.args.length === 0 && row.os.length === 0);\n  expect(rows.length).toBeGreaterThanOrEqual(15);\n  for (const row of rows) expect(resolve(row.environment), JSON.stringify(row)).toBe(row.expected?.language ?? null);\n  for (const row of vectors.choiceOrders) expect(resolve({}), JSON.stringify(row)).toBe(row.expected);")
    edit(D + "🧪️tests/🌐️language/🟦️.ts", ts_test)
    import subprocess, re as _re
    listed = grep_files("SEMIO_LOCALE", D, D)
    for rel in filter(None, listed):
        edit(rel, lambda text: _re.sub(r"SEMIO_LOCALE\b", "SEMIO_LANGUAGE", text))
    edit(D + "README.md", lambda text: replace(text, "the language is the one the system names (`LC_ALL`,\n`LC_MESSAGES`, `LANG`). When no supported system or explicit preference resolves a language,", "the language is the one the repository language protocol names (`SEMIO_LANGUAGE`, which `repo` exports with its choice,\nthen `LANGUAGE`, then the first set of `LC_ALL`, `LC_MESSAGES`, `LANG`). When no supported protocol source or explicit preference resolves a language,"))
    return out


def routing(base):
    """🛤️ Progress lines, first-failure notices and the native heartbeat follow the one protocol: a line is written in the chosen language, in every offered language when none is chosen (the English fallback is gone); the schema copies of the variable order (`systemVariables`, `fallback`) and the retired SEMIO_DASHBOARD_LOCALE are gone; the fixture rows that relied on the fallback name their language."""
    out, P_ = {}, F + "🔨️modules/🏃️process/"
    R = P_ + "🧭️routing/📥️invocation/🏃️process/"
    def edit(rel, fn):
        out[rel] = fn(out[rel].decode() if rel in out else base(rel).decode()).encode()
    def process_ts(text):
        text = swap(text, "function progressLanguage(", 'function progressLanguage(environment:Readonly<Record<string,string|undefined>>):"en"|"de"{return noticeLanguagesV1(["en","de"] as const,environment)[0]!;}')
        text = swap(text, "export function firstFailureLanguagesV1(", 'export function firstFailureLanguagesV1(environment:Readonly<Record<string,string|undefined>>):readonly("en"|"de")[]{return noticeLanguagesV1(Object.keys(progressSchema.const.firstFailure.labels) as ("en"|"de")[],environment);}')
        sig = "export function formatScriptProcessActivityV1(progress:ScriptProgress,environment:Readonly<Record<string,string|undefined>>=process.env):string|null{\n"
        text = replace(text, sig, sig + ' const languages=noticeLanguagesV1(["en","de"] as const,environment);if(languages.length>1){const lines=languages.map(language=>formatScriptProcessActivityV1(progress,{...environment,[LANGUAGE_PROTOCOL_V1.explicitVariable]:language}));return lines.includes(null)?null:lines.join("\\n");}\n')
        text = replace(text, 'if(row.message!==null)write("[script] "+row.message+"\\n");};', 'if(row.message!==null)for(const line of row.message.split("\\n"))write("[script] "+line+"\\n");};')
        return add_import(text, R + "🟦️.ts", ["LANGUAGE_PROTOCOL_V1", "noticeLanguagesV1"])
    edit(R + "🟦️.ts", process_ts)
    def schema(text):
        data = json.loads(text)
        for key in ("sourceAdmission", "firstFailure"):
            data["const"][key].pop("systemVariables")
        data["const"]["sourceAdmission"].pop("fallback")
        return json.dumps(data, ensure_ascii=False, indent=2) + "\n"
    edit(R + "🧬️schema/📈️progress.json", schema)
    VARS = "[LANGUAGE_PROTOCOL_V1.explicitVariable,LANGUAGE_PROTOCOL_V1.preferenceListVariable,...LANGUAGE_PROTOCOL_V1.environmentOrder]"
    T = R + "🧪️tests/📈️progress/🟦️.ts"
    def progress_test(text):
        text = text.replace("contract.const.sourceAdmission.systemVariables", "languageVariables").replace("declaration.systemVariables", "languageVariables").replace("locale.systemVariables", "languageVariables")
        text = text.replace("JSON.stringify(locale.fallback)", '"[]"').replace("JSON.stringify(declaration.fallback)", '"[]"')
        text = replace(text, 'const language=/^de/u.test(row.environment.LANG??row.environment.SEMIO_DASHBOARD_LOCALE??"")?"de":"en",', 'const language=noticeLanguagesV1(["en","de"] as const,row.environment)[0]!,')
        text = replace(text, 'language=row.environment.LANG==="de"?"de":"en",', 'language=noticeLanguagesV1(["en","de"] as const,row.environment)[0]!,', 3)
        if "systemVariables" in text or ".fallback" in text or "SEMIO_DASHBOARD_LOCALE" in text: raise SystemExit("progress test: a copy of the variable order is left")
        text = add_import(text, T, ["LANGUAGE_PROTOCOL_V1", "noticeLanguagesV1"])
        text = replace(text, 'import fixture from "./🔣️.json";\n', 'import fixture from "./🔣️.json";\n\nconst languageVariables = ' + VARS + ';\n')
        return text + ('\ntest("a progress line without a chosen language is written in every offered language, one line each",()=>{\n'
                       ' const row=fixture.sourceCases.find(candidate=>candidate.expected!==null)!,progress:ScriptProgress={owner:"process-progress-fixture",command:"compiler",stage:"running",activity:{resource:row.resource,mode:"shared",elapsedMs:0}};\n'
                       ' const en=formatScriptProcessActivityV1(progress,{SEMIO_LANGUAGE:"en"}),de=formatScriptProcessActivityV1(progress,{SEMIO_LANGUAGE:"de"});\n'
                       ' expect(noticeLanguagesV1(["en","de"] as const,{LANG:"C"})).toEqual(["en","de"]);expect(en).not.toBe(de);\n'
                       ' for(const environment of [{},{LANG:"C"},{LC_ALL:"fr_FR.UTF-8",LANG:"de_DE.UTF-8"},{SEMIO_LANGUAGE:"fr"}])expect(formatScriptProcessActivityV1(progress,environment),JSON.stringify(environment)).toBe(`${en}\\n${de}`);\n'
                       '});\n')
    edit(T, progress_test)
    def fixture(text):
        data, changed = json.loads(text), 0
        for key in ("parserCases", "queueCases", "queueHolderCases", "sourceCases", "cacheCases", "deleteCases", "providerCases", "toolingCases"):
            for row in data.get(key, []):
                environment = row.get("environment")
                if not isinstance(environment, dict): continue
                if "SEMIO_DASHBOARD_LOCALE" in environment:
                    retired = environment.pop("SEMIO_DASHBOARD_LOCALE"); changed += 1
                    if choose_notice({"SEMIO_LANGUAGE": retired}) is not None: environment.setdefault("SEMIO_LANGUAGE", retired)
                spoken = choose_notice(environment)
                if spoken is None: environment["SEMIO_LANGUAGE"] = "en"; changed += 1
        if changed < 30: raise SystemExit(f"progress fixture: only {changed} rows named their language")
        return json.dumps(data, ensure_ascii=False, indent=2) + "\n"
    edit(R + "🧪️tests/📈️progress/🔣️.json", fixture)
    E = R + "⏲️expiry/🧪️tests/⏳️progress/🟦️.ts"
    edit(E, lambda text: add_import(replace(text, "const languageVariables = progressSchema.const.firstFailure.systemVariables,", "const languageVariables = " + VARS + ","), E, ["LANGUAGE_PROTOCOL_V1"]))
    edit(RT + "🧪️tests/🟦️.ts", lambda text: replace(text, 'import { existsSync, readFileSync } from "node:fs";', 'import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";') + src("scan-law.ts").decode())
    O = P_ + "🎛️owned-execution/"
    edit(O + "🟦️.ts", lambda text: add_import(swap(text, "export function nativeProgressLanguagesV1(", '/** 🌐️ The languages a heartbeat is written in: the one the language protocol chooses, English and German when it chooses none. */\nexport function nativeProgressLanguagesV1(environment: Readonly<Record<string, string | undefined>>): readonly ("en" | "de")[] {\n  return noticeLanguagesV1(heartbeat.const.languages as readonly ("en" | "de")[], environment);\n}'), O + "🟦️.ts", ["noticeLanguagesV1"]))
    edit(O + "🧬️schema/📣️heartbeat.json", lambda text: replace(text, '    "systemVariables": [\n      "SEMIO_LANGUAGE",\n      "LANGUAGE",\n      "LC_ALL",\n      "LC_MESSAGES",\n      "LANG"\n    ],\n', ""))
    edit(O + "🧪️tests/🟦️.ts", lambda text: replace(text, '[{ LC_ALL: "C", LANG: "en_US.UTF-8" }, ["en"]]', '[{ LC_ALL: "C", LANG: "en_US.UTF-8" }, ["en", "de"]], [{ LANGUAGE: "fr:de" }, ["de"]]'))
    return out


def choose_notice(environment):
    """🐍️ The protocol's notice language of one environment, by the independent oracle (None when it chooses none)."""
    spec = importlib.util.spec_from_file_location("lp_oracle", os.path.join(SRC, "oracle.py"))
    oracle = importlib.util.module_from_spec(spec); spec.loader.exec_module(oracle)
    got = oracle.choose(json.loads(src("protocol.json")), ["en", "de"], [], environment, [])
    return got.get("language")


BATCHES = {"core": core, "copies": copies, "imports": imports, "launcher": launcher, "launcher2": launcher2, "dashboard": dashboard, "routing": routing}


def write_batch(name, edits, base):
    folder = os.path.join(HERE, "staged", name)
    if os.path.exists(folder): shutil.rmtree(folder)
    os.makedirs(os.path.join(folder, "files")); os.makedirs(os.path.join(folder, "base"))
    manifest, modes = {}, {}
    for index, (rel, data) in enumerate(sorted(edits.items())):
        before = base(rel)
        if before == data: continue
        file = f"{index:03d}" + os.path.splitext(rel)[1]
        if data is not None: open(os.path.join(folder, "files", file), "wb").write(data)
        if before is not None:
            open(os.path.join(folder, "base", file), "wb").write(before)
            live = os.path.join(ROOT, rel)
            if os.path.exists(live): modes[rel] = oct(stat.S_IMODE(os.stat(live).st_mode))
        manifest[rel] = {"file": file, "base": sha(before), "new": sha(data)}
    json.dump(manifest, open(os.path.join(folder, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    json.dump(modes, open(os.path.join(folder, "modes.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    shutil.copy(os.path.join(HERE, "stage.py"), os.path.join(folder, "stage.py"))
    open(os.path.join(folder, "batch.txt"), "w", encoding="utf8").write((BATCHES[name].__doc__ or "").strip() + "\n")
    verify = os.path.join(HERE, "verify", name + ".sh")
    if os.path.exists(verify): shutil.copy(verify, os.path.join(folder, "verify.sh"))
    print(f"{name}: {len(manifest)} files -> {folder}")


def main():
    names = [a for a in sys.argv[1:] if not a.startswith("--") and a != ROOT] or list(BATCHES)
    table = pending_base()
    base = lambda rel: table[rel] if rel in table else read_live(rel)
    for name, make in BATCHES.items():
        edits = make(base)
        if name in names: write_batch(name, edits, base)
        table.update(edits)


main()
