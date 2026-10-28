"""🏗️ [td-ucache] rebuilds the staged batch `scratch` from the LIVE tree: unit-cache tests that start Cargo create their scratch workspaces below `cargoScratchBaseV1` (outside every enclosing Cargo configuration, workspace and toolchain file) instead of below a SEMIO_TEST_ARTIFACT_DIR that may lie inside the checkout.
usage: python3 -I build.py [--root LIVE]   -> staged/scratch/{manifest.json,files,base,stage.py,verify.sh}; exit 2 when an anchor is not found exactly once."""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else HERE.split("/.🧬semio/")[0]
U = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/"
OUT = os.path.join(HERE, "staged", "scratch")
NEW = {U + "🧪️tests/🏖️scratch/🟦️.ts": "scratch.ts", U + "🧪️tests/🏖️scratch/🔣️.json": "scratch.json", U + "🧪️tests/🏖️scratch/🧪️tests/🟦️.ts": "scratch-law.ts"}
OLD_BASE = 'const base=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir());mkdirSync(base,{recursive:true});'
EDITS = {
    U + "🧪️tests/🟦️.ts": [
        ('import {tmpdir} from "node:os";\n', ''),
        ('import {CATALOG_LANGUAGES_V1}', 'import {cargoScratchBaseV1} from "./🏖️scratch/🟦️.ts";\nimport {CATALOG_LANGUAGES_V1}'),
        ('Every Cargo child runs with the scratch workspace as its working directory, so no repository `.cargo/config.toml` can leak in.', 'Every Cargo child runs in a scratch workspace below `cargoScratchBaseV1`, outside every enclosing Cargo configuration, workspace manifest and toolchain file, so no repository `.cargo/config.toml` can leak in even when the test artifact directory lies inside a checkout.'),
        (OLD_BASE, 'const base=cargoScratchBaseV1();'),
    ],
    U + "🧪️tests/🌱️incremental/🟦️.ts": [
        ('import {tmpdir} from "node:os";\n', ''),
        ('import fixture from "./🔣️.json" with {type:"json"};\n', 'import fixture from "./🔣️.json" with {type:"json"};\nimport {cargoScratchBaseV1} from "../🏖️scratch/🟦️.ts";\n'),
        (',base=resolve(tmpdir());', ',base=cargoScratchBaseV1();'),
        ('The scratch workspace lives in the system temporary directory, outside every checkout, so no repository `.cargo/config.toml` reaches Cargo.', 'The scratch workspace lives below `cargoScratchBaseV1`, outside every enclosing Cargo configuration, workspace manifest and toolchain file, so no repository `.cargo/config.toml` reaches Cargo.'),
    ],
    U + "🎰️jobserver/🧪️tests/🟦️.ts": [
        ('import {tmpdir} from "node:os";\n', ''),
        ('import {join,resolve} from "node:path";\n', 'import {join} from "node:path";\n'),
        ('import {buildJobserverProbeV1,type ProbeVerdictV1} from "./🦀️probe/🟦️.ts";\n', 'import {buildJobserverProbeV1,type ProbeVerdictV1} from "./🦀️probe/🟦️.ts";\nimport {cargoScratchBaseV1} from "../../🧪️tests/🏖️scratch/🟦️.ts";\n'),
        (OLD_BASE, 'const base=cargoScratchBaseV1();'),
    ],
    U + "🎰️jobserver/🧪️tests/🚦️pool/🟦️.ts": [
        ('import {tmpdir} from "node:os";\n', ''),
        ('import {buildJobserverProbeV1,cargoWithRegistryV1,plainCargoEnvironmentV1,type ProbeVerdictV1} from "../🦀️probe/🟦️.ts";\n', 'import {buildJobserverProbeV1,cargoWithRegistryV1,plainCargoEnvironmentV1,type ProbeVerdictV1} from "../🦀️probe/🟦️.ts";\nimport {cargoScratchBaseV1} from "../../../🧪️tests/🏖️scratch/🟦️.ts";\n'),
        (OLD_BASE, 'const base=cargoScratchBaseV1();'),
    ],
    U + "🧵️lane/📤️retention/🧪️tests/🧾️provenance-equality/🟦️.ts": [
        ('import {tmpdir} from "node:os";\n', ''),
        ('import {join,resolve} from "node:path";\n', 'import {join} from "node:path";\n'),
        ('import {retainLaneOutputsV1} from "../../🟦️.ts";\n', 'import {retainLaneOutputsV1} from "../../🟦️.ts";\nimport {cargoScratchBaseV1} from "../../../../🧪️tests/🏖️scratch/🟦️.ts";\n'),
        (OLD_BASE, 'const base=cargoScratchBaseV1();'),
    ],
}
def sha(data): return hashlib.sha256(data).hexdigest()
def main():
    shutil.rmtree(OUT, ignore_errors=True)
    for sub in ("files", "base"): os.makedirs(os.path.join(OUT, sub))
    manifest, bad, n = {}, [], 0
    for rel, edits in EDITS.items():
        n += 1; name = "f%03d" % n
        raw = open(os.path.join(ROOT, rel), "rb").read(); text = raw.decode("utf8")
        for old, new in edits:
            count = text.count(old)
            if count != 1: bad.append("%s x%d in %s" % (old[:60], count, rel[-70:])); continue
            text = text.replace(old, new)
        data = text.encode("utf8")
        open(os.path.join(OUT, "base", name), "wb").write(raw); open(os.path.join(OUT, "files", name), "wb").write(data)
        manifest[rel] = {"file": name, "base": sha(raw), "new": sha(data)}
    for rel, src in NEW.items():
        n += 1; name = "f%03d" % n
        if os.path.exists(os.path.join(ROOT, rel)): bad.append("exists: " + rel)
        data = open(os.path.join(HERE, "src", src), "rb").read()
        open(os.path.join(OUT, "files", name), "wb").write(data)
        manifest[rel] = {"file": name, "base": None, "new": sha(data)}
    if bad:
        print("\n".join("ANCHOR " + b for b in bad)); sys.exit(2)
    json.dump(manifest, open(os.path.join(OUT, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    shutil.copy(os.path.join(HERE, "stage.py"), os.path.join(OUT, "stage.py")); shutil.copy(os.path.join(HERE, "verify.sh"), os.path.join(OUT, "verify.sh"))
    print("staged %d files (%d edited, %d new) from %s" % (len(manifest), len(EDITS), len(NEW), ROOT))
main()
