"""🔁 Stages the held-lane retry for a single Cargo pass with several components."""
from pathlib import Path
import argparse
import hashlib
import json

ROOT = Path("🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass")
TEST = Path("🔁️failure-churn/🧪️tests")
OUTPUT = Path(__file__).resolve().parent / "🗑️generated" / "component-retry"


def replace(text, before, after):
    """📍 Replace the reviewed live anchor."""
    assert text.count(before) == 1, before
    return text.replace(before, after, 1)


def candidates(test_only):
    """🧪 Select the fixture-first or integrated change against current source."""
    files = {}
    def change(path, edits):
        file = ROOT / path
        before = file.read_bytes()
        text = before.decode("utf8")
        for expected, updated in edits:
            text = replace(text, expected, updated)
        files[file] = (before, text.encode("utf8"))

    change(TEST / "🔣️.json", [
        ('{"generation":"warm","passes":[{"profile":"prototype","components":2}],"enabled":false}',
         '{"generation":"warm","passes":[{"profile":"prototype","components":2}],"enabled":true}'),
        ('{"generation":"warm","passes":[],"enabled":false}',
         '{"generation":"warm","passes":[{"profile":"prototype","components":0}],"enabled":false},\n    {"generation":"warm","passes":[],"enabled":false}'),
        ('"application":"a","prior"', '"application":"a","secondary":"d","prior"'),
        ('"finalExit":0,"artifacts":2}', '"finalExit":0,"artifacts":2,"storageBytes":5368709120}'),
    ])
    change(TEST / "🟦️.ts", [
        ("sum(json_extract(value, '$.components')) = 1", "sum(json_extract(value, '$.components')) > 0"),
        *([] if test_only else [("import {admitWarmCargoRetryGraphV1,failedDependencyDriftV1,retryableSingleComponentPassV1,", "import {admitWarmCargoRetryGraphV1,failedDependencyDriftV1,retryableWarmComponentPassV1,"),
                               ("expect(retryableSingleComponentPassV1(", "expect(retryableWarmComponentPassV1(")]),
    ])
    if test_only:
        return files

    change(Path("🔁️failure-churn/🟦️.ts"), [
        ("A warm single-component pass can retry before publishing any linked output.", "A single warm Cargo pass can retry before publishing any linked component."),
        ("retryableSingleComponentPassV1",
         "retryableWarmComponentPassV1"),
        ("passes[0]?.packages.length === 1", "(passes[0]?.packages.length ?? 0) > 0"),
    ])
    change(Path("🟦️.ts"), [
        ("   const retrySingleWarm = retryableSingleComponentPassV1", "  const retryWarmPlan = retryableWarmComponentPassV1"),
        ("failedPassConcurrentEditV1,retryableSingleComponentPassV1,", "failedPassConcurrentEditV1,retryableWarmComponentPassV1,"),
        ('retrySingleWarm ? ["", "-retry"]', 'retryWarmPlan ? ["", "-retry"]'),
        ("if (retrySingleWarm) requireMaterializedPathV1", "if (retryWarmPlan) requireMaterializedPathV1"),
        ("const args = [...inputs.arguments], signal=effectControl.signal;", "const args = [...inputs.arguments], signal=effectControl.signal;\n    const retryWarm = retryWarmPlan && pass.id === plan.passes[0]?.id;"),
        ("const compilerErrors = new Map<string,Set<string>>()", "let stream = !retryWarm;\n        const compilerErrors = new Map<string,Set<string>>()"),
        ("if (!retrySingleWarm) prefetch(message);", "if (stream) prefetch(message);"),
        ("if (component && !linked.has(component.package.name)) { linked.set(component.package.name, component.wasm); if (!retrySingleWarm) streamComponent(pass, component.package, component.wasm); }", "if (component && !linked.has(component.package.name)) {\n                    linked.set(component.package.name, component.wasm);\n                    if (!stream && pass.packages.length > 1) {\n                      stream = true;\n                      for (const unit of units) prefetch(unit.message);\n                    }\n                    if (stream) streamComponent(pass, component.package, component.wasm);\n                  }"),
        ("if (!retrySingleWarm || failed.exit", "if (!retryWarm || failed.exit"),
        ("if (!held||!unitGraphClosuresV1(graph,pass).get(pass.packages[0]!.name)?.size) return false;",
         "const closures=unitGraphClosuresV1(graph,pass);\n              if (!held||pass.packages.some(row=>!closures.get(row.name)?.size)) return false;"),
        ("const [replacement]=componentBatchesV1(pass,graph,history,budget,", "const replacements=componentBatchesV1(pass,graph,history,budget,"),
        ("return !!replacement&&replacement.bytes<=budget;", "return replacements.length===1&&replacements[0]!.bytes<=budget;"),
        ("if (retrySingleWarm) {", "if (retryWarm && pass.packages.length === 1) {"),
    ])
    change(Path("🧪️tests/🟦️.ts"), [
        ("verifyStageWriters?:boolean } = {}, mode", "verifyStageWriters?:boolean; storageBytes?:number } = {}, mode"),
        ("const { cancel, verifyStageWriters, ...driver } = overrides;", "const { cancel, verifyStageWriters, storageBytes, ...driver } = overrides;"),
        ("withLawStorageV1(control, mode === \"observe-linked\" ? lawStoragePolicyV1.realImageBytes : lawStoragePolicyV1.stubImageBytes,", "withLawStorageV1(control, storageBytes ?? (mode === \"observe-linked\" ? lawStoragePolicyV1.realImageBytes : lawStoragePolicyV1.stubImageBytes),"),
        ("log:line=>seen.push(line),\n    stages:", "log:line=>seen.push(line),\n    storageBytes:vector.storageBytes,\n    stages:"),
        ("the held native lane retries a stable locked dependency change", "the held native lane retries a stable locked dependency change for two components"),
        ("[vector.application,vector.prior,vector.replacement]", "[vector.application,vector.secondary,vector.prior,vector.replacement]"),
        ("const pass={...original,packages:[original.packages[0]!]}, workspace=dirname(pass.manifest);", "const pass={...original,packages:original.packages.slice(0,2)}, workspace=dirname(pass.manifest);"),
        ("const manifest=(dependency:string)=>[", "const manifest=(name:string,dependency:string)=>["),
        ("'name = \"'+vector.application+'\"'",
         "'name = \"'+name+'\"'"),
        ("writeFileSync(path(vector.application,\"Cargo.toml\"),manifest(vector.prior));",
         "for (const row of pass.packages) writeFileSync(row.manifest,manifest(row.name,vector.prior));"),
        ("manifest:manifest(vector.replacement),header:",
         "manifests:pass.packages.map(row=>[row.manifest,manifest(row.name,vector.replacement)]),header:"),
        ("const {vector:v,workspace:w,calls,target,priorChecksum,manifest,header}",
         "const {vector:v,workspace:w,calls,target,priorChecksum,manifests,header}"),
        ("const units=[unit(v.application,\"cdylib\",[\"default\"],[1]),unit(child,\"rlib\",[],[])];",
         "const units=[unit(v.application,\"cdylib\",[\"default\"],[1]),unit(child,\"rlib\",[],[]),unit(v.secondary,\"cdylib\",[\"default\"],[1])];"),
        ("JSON.stringify({version:1,units,roots:[0]})",
         "JSON.stringify({version:1,units,roots:[0,2]})"),
        ("writeFileSync(app,manifest);",
         "for (const [path, manifest] of manifests) writeFileSync(path,manifest);"),
        ("announce(units[0],join(directory,v.application.replaceAll(\"-\",\"_\")+\".wasm\"),undefined);",
         "for (const index of [0,2]) announce(units[index],join(directory,units[index].target.name.replaceAll(\"-\",\"_\")+\".wasm\"),undefined);"),
        (".toEqual([vector.application,vector.replacement]);",
         ".toEqual([vector.application,vector.replacement,vector.secondary]);"),
        ("expect(outcome.staged).toEqual([vector.application+\":release\"]);",
         "expect([...outcome.staged].sort()).toEqual([vector.application+\":release\",vector.secondary+\":release\"]);"),
    ])
    return files


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--test-only", action="store_true")
    parser.add_argument("--apply", action="store_true")
    args = parser.parse_args()
    files = candidates(args.test_only)
    OUTPUT.mkdir(parents=True, exist_ok=True)
    review = OUTPUT / "receipt.json"
    receipt = {}
    for index, (file, (before, after)) in enumerate(files.items()):
        assert file.read_bytes() == before, file
        assert len(str(file.resolve()).encode("utf-16-le")) // 2 <= 256, file
        destination = OUTPUT / f"{index}{file.suffix}"
        receipt[str(file)] = [destination.name, hashlib.sha256(before).hexdigest(), hashlib.sha256(after).hexdigest()]
        if args.apply:
            assert destination.read_bytes() == after, file
        else:
            destination.write_bytes(after)
    if args.apply:
        assert json.loads(review.read_text()) == receipt, "source changed after review"
        for file, (before, after) in files.items():
            assert file.read_bytes() == before, file
            file.write_bytes(after)
    else:
        review.write_text(json.dumps(receipt, ensure_ascii=False, indent=2) + "\n")
    print(f"[DEBUG] Verified {len(files)} component retry source preimages")
