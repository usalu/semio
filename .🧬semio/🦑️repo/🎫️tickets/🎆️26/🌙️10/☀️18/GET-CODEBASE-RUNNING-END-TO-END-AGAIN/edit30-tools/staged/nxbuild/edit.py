"""📋 Anchored, idempotent edits of batch `nxbuild` on a root: edit.py <root> check|apply. The Nextest build step reads the planned `cargo metadata` too (`--cargo-metadata`, no `--manifest-path`) when the metadata describes the request's own workspace manifest, so Nextest starts only `cargo test --no-run` and no `cargo metadata` of its own; the steps law renders that plan as well."""
import os, sys
root, verb = sys.argv[1], sys.argv[2]
P = os.path.join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts")
S = os.path.join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🧪️tests/🪜️steps/🟦️.ts")
EDITS = {P: [
 ("/** 🧭️ Projects one exact Cargo request into bounded build, assertion and coverage operations. */\nexport function cargoTestPlanV1(request:CargoTestRequestV1, input:CargoTestPolicyV1, metadataPath:string, cargoMetadataPath?:string):CargoTestStepV1[] {",
  "/** 🧭️ Projects one exact Cargo request into bounded build, assertion and coverage operations. With `cargoMetadataPath` (the `cargo metadata` Cargo planning resolved) the Nextest assertion reads it instead of the manifest, and so does the Nextest build when `metadataManifest` (the workspace manifest that metadata describes) is the request's manifest: Nextest then builds that workspace's manifest with the request's packages and starts no `cargo metadata` of its own.\n * @see https://nexte.st/docs/ci-features/reusing-builds/ */\nexport function cargoTestPlanV1(request:CargoTestRequestV1, input:CargoTestPolicyV1, metadataPath:string, cargoMetadataPath?:string, metadataManifest?:string):CargoTestStepV1[] {"),
 ("||cargoMetadataPath!==undefined&&!isAbsolute(cargoMetadataPath)||",
  "||cargoMetadataPath!==undefined&&!isAbsolute(cargoMetadataPath)||metadataManifest!==undefined&&!isAbsolute(metadataManifest)||"),
 ('  if(policy.nextest) return [step("build",["nextest","list",...compiler,"--list-type","binaries-only","--message-format","json",...profile,...packages,...split.buildArgs],true),',
  '  const planned=cargoMetadataPath!==undefined&&metadataManifest===request.manifestPath?["--cargo-metadata",cargoMetadataPath,...request.packages.flatMap(name=>["-p",name])]:packages;\n  if(policy.nextest) return [step("build",["nextest","list",...compiler,"--list-type","binaries-only","--message-format","json",...profile,...planned,...split.buildArgs],true),'),
 ("try{const plan=cargoTestPlanV1(request,policy,metadata,cargoMetadata),steps",
  "try{const root=(planned as {workspace_root?:unknown}|null)?.workspace_root,plan=cargoTestPlanV1(request,policy,metadata,cargoMetadata,cargoMetadata&&typeof root===\"string\"?join(root,\"Cargo.toml\"):undefined),steps"),
], S: [
 ("const steps=[...cargoTestPlanV1(request,policy,fixture.metadata),...cargoTestPlanV1(request,policy,fixture.metadata,fixture.cargoMetadata)].map(",
  "const steps=[...cargoTestPlanV1(request,policy,fixture.metadata),...cargoTestPlanV1(request,policy,fixture.metadata,fixture.cargoMetadata),...cargoTestPlanV1(request,policy,fixture.metadata,fixture.cargoMetadata,policy.manifestPath)].map("),
]}
bad = 0
for path, edits in EDITS.items():
    text = open(path, encoding="utf8").read(); new = text
    for old, rep in edits:
        if new.count(old) == 1: new = new.replace(old, rep)
        elif new.count(rep) == 1: pass
        else: print("ANCHOR", os.path.basename(os.path.dirname(path)), repr(old[:70])); bad += 1
    print("same" if new == text else "edit", path[-60:])
    if verb == "apply" and not bad and new != text:
        mode = os.stat(path).st_mode & 0o777; tmp = path + ".e30"; open(tmp, "w", encoding="utf8").write(new); os.chmod(tmp, mode); os.replace(tmp, path)
sys.exit(1 if bad else 0)
