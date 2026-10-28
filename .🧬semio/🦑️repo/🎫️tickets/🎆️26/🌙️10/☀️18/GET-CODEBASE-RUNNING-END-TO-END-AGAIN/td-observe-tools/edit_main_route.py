"""📂️ [td-observe] (1) the main observation law resolves its artifact root physically once, so a bare `bun test ./🟦️.ts` gives the truth
(observation refuses a symlinked ancestry: the macOS temporary directory lives below the `/var` symlink, 0 pass / 1 fail before); the
settlement laws fall back to the same root instead of refusing. (2) the build suite route table births the two Node-child project laws
(`📦️projects`, `⚡️incremental`) with the immutable Nx SDK (`setup-sdk`), as `🧭️selection/📥️closure` and `📥️inputs/🎯️selected` already are:
without `SEMIO_NX_TOOLING_IDENTITY` they refuse "Original artifact namespace and immutable SDK required" on every runner.
usage: python3 -I edit_main_route.py <root> (idempotent, anchored)."""
import json, sys

ROOT = sys.argv[1]
LIB = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
MAIN = LIB + "👁️observation/🧪️tests/🟦️.ts"
HELPER = 'const artifactRoot = (() => { const root = process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(); mkdirSync(root, { recursive: true }); return realpathSync(root); })();\n'
VALIDATE = 'const validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json", import.meta.url), "utf8")));\n'
with open(MAIN, encoding="utf8") as handle:
    text = handle.read()
before = text
if HELPER not in text:
    pairs = [
        (VALIDATE, VALIDATE + "/** 📂️ The physical root of every law workspace (SEMIO_TEST_ARTIFACT_DIR, else the OS temporary directory), resolved once: observation refuses a symlinked ancestry such as the macOS `/var`. */\n" + HELPER),
        ('const root=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir();mkdirSync(root,{recursive:true});await observeInference({cachePath:join(root,', 'await observeInference({cachePath:join(artifactRoot,'),
        ('  const parent = process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir();\n  mkdirSync(parent, { recursive: true });\n  const root = mkdtempSync(join(realpathSync(parent), "io-"));', '  const root = mkdtempSync(join(artifactRoot, "io-"));'),
        ('parent=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!parent)throw Error("Original Source law artifact directory required");', 'parent=artifactRoot;'),
    ]
    for old, new in pairs:
        if text.count(old) != 1:
            raise SystemExit(f"refused: anchor {old[:50]!r}")
        text = text.replace(old, new)
    if text.count("process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()") != 4:
        raise SystemExit("refused: expected four remaining inline roots")
    text = text.replace("process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()", "artifactRoot")
    if text.count("process.env.SEMIO_TEST_ARTIFACT_DIR") != 1:
        raise SystemExit("refused: leftover artifact roots")
if text != before:
    with open(MAIN, "w", encoding="utf8") as handle:
        handle.write(text)
print(("edited " if text != before else "already ") + "observation main law")

TABLE = LIB + "🗂️suites/🔣️.json"
with open(TABLE, encoding="utf8") as handle:
    raw = handle.read()
data = json.loads(raw)
row = next(suite for suite in data["suites"] if suite["id"] == "build-observation")
launchers = {"👁️observation/🧪️tests/⚡️incremental/🟦️.ts": "setup-sdk", "👁️observation/🧪️tests/📦️projects/🟦️.ts": "setup-sdk"}
if row.get("fileLaunchers") != launchers:
    if "fileLaunchers" in row:
        raise SystemExit("refused: build-observation already routes other files")
    ordered = {}
    for key, value in row.items():
        ordered[key] = value
        if key == "trees":
            ordered["fileLaunchers"] = launchers
    row.clear()
    row.update(ordered)
out = json.dumps(data, ensure_ascii=False, indent=2) + "\n"
if out != raw:
    with open(TABLE, "w", encoding="utf8") as handle:
        handle.write(out)
print(("edited " if out != raw else "already ") + "build suite route table")
