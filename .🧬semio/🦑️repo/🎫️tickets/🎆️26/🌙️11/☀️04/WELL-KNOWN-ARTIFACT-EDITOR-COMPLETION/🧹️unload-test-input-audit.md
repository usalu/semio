# Unload Test Input Review

The neutral corpus has six required-field scenarios: unbound artifact, documentless tool, missing attached status, saved attached status, pending attached status, failed attached status. Independent Ajv validates the corpus and independently checks the Boolean oracle. Actual hook tests dispatch cancelable beforeunload and inspect defaultPrevented, including background spawned editor transitioning to saved and same numeric instance on a different handle.

Suggested additional strictness cases: same handle and numeric instance but different app ID must not lend attachment status; persisted=false with pendingMutations=0 must still block. The status enum compresses these actual wire fields, so the pure enum oracle alone does not verify strict saved-state derivation. Missing status must be checked as kind===saved rather than !isUnsaved, since isUnsaved intentionally returnsfalse for unknown.

The new production helper was not yet present at this review checkpoint; root waits for the red test before implementation. No passing helper claim is made.

Scoped read-only diff review found33tracked renderer changes; none exceed256UTF16codeunits as absolute workspace paths. Nonhost helper changes are the intended saved-state close fencing and transfer cancellation/reopen notices. This does not certify unrelated simultaneous repository changes.

## First Implementation Review

The helper now exists. Exact handle/instance/app matching is correct; every actual attachment requires strict saved derivation, missing statuses block, and capture failures conservatively block. Concrete test-data blind spot found: helper classifies artifactSchema!==undefined as artifact, while a real AppDefinition always has io.artifactSchema:string and documentless surfaces commonly use empty string. The documentless mock omitted io, hiding a false warning for Home/tools. Reported to root: require nonempty schema and represent documentless metadata as io:{artifactSchema:""}. This is a source finding, not an executed failing assertion.

## Corrected Final Source Review

Root reports expanded tests reproduced two actual failures before fixes: empty schema tool classification and direct detach missing status. Source now checks nonempty schema length. The corpus uses real io:{artifactSchema:""} for documentless metadata and includes exact app/instance mismatch inputs. Parent host-mode57 calls the helper with actual loaded/session/attachment/status values. Attached owners require kind===saved and exact handle/app/instance matching; empty-schema tools with no attachment remain unblocked.

Closehook163–166 now normalizes unknown status to unsaved only when a captured real attachment exists, and162 requires both exact captured map entry and current request controller. Missing attachment detach behavior remains distinct. No concrete regression found in this narrow source review. Reserved768MB focused rerun session76244 is queued; no pass claim for the corrected helper until it completes.
