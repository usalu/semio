# Print check producer

- Root cause: `readTypeScriptPackageProducerV1` returned undefined because `📋️project.json` of `@semio-tech/print` lacked `metadata.semio.artifactPackage`; `withCompilation` (caching/📦️artifacts/🟦️typescript/🟦️.ts:32) then threw "Authored TypeScript artifact producer required". The print `CheckScript` uses `runArtifactTypeScriptPackageMain`, which requires one (as the norm packages declare).
- Fix 1: added `{kind:"typescript-artifact-package",source:"📦️artifacts/🟦️.ts",target:"check"}` to print `📋️project.json`.
- Fix 2: real type error TS2339 in `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧹️retirement/🟦️.ts:25`: `observed.identity!.target` -> `observed.target`.
- Proof: `🗑️generated/print-check-producer-2.log` exit 0.
