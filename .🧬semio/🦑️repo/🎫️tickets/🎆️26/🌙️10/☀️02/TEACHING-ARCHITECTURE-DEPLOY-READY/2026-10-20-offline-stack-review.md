# Offline Stack Review And Prepared RED

Read-only deployment review plus narrowly authorized actual test inputs. No build/test command, download, Docker action, stack, push or upload ran. Frozen native helper and CI workflow are unchanged.

## Concrete Defect

stackBundlePlan rewrites only Caddy to pull_policy:never; actual staged proctor remains missing. This contradicts its generated README promise that nothing is pulled. Compose missing can consult a registry, including latest behavior; never uses the local image or fails. Independent audit verified the official [Compose pull_policy contract](https://docs.docker.com/reference/compose-file/services/#pull_policy). Changing the staged bundle only preserves online/registry compose semantics.

## Actual RED Inputs

Added the closed neutral Offline Stack Image Policy schema before its fixture/test changes. Existing bundle fixture now declares proctor never alongside the existing Caddy digest-tag/never policy, plus one authored proctor-policy refusal vector. The existing actual producer test parses stackBundlePlan.compose with third-party yaml, compares whole services against the authored compose plus neutral expected offline policy, and independently validates fixture/parsed result through Ajv. It therefore protects env, volumes, security and image interpolation while requiring both offline policies. No new test module/target/runtime dependency was introduced. Current first staged-proctor comparison must fail missing versus never once the architecture suite actually collects; the earlier shell:typegen precollection refusal is not RED. No assertion outcome is claimed.

Production source remains held. Minimal candidate patch is [offline-stack.patch](🎯️contracts/offline-stack.patch): require exact authored proctor policy, replace only that block with never, retain existing Caddy transformation and all image tags, then clarify template/runbook text for both services. It introduces no migration, generic transport or online template change. Apply only after intended assertion RED, then rerun the same owner to GREEN.

## Preserved Deployment Contract

The template/runbook already take a backup first, retain the operator .env and certificates/, set only PROCTOR_TAG to the exact new bundle tag, load the archive, use compose up --detach --wait, and check /instance. They retain the directory/project name and named data/certificate volumes. The staging verb itself writes its output .env; choose a fresh staging directory, never point --out at the live operator stack. No source behavior was changed for that boundary.

Image producer already requires actual native Linux x64 release, retains the original compiler artifact, validates selected course content, checks ELF64 machine against the genuine producer, sends a finite runtime-only context, and executes final nonroot runtime proctor check inside the pinned Debian12 image. A native repo ELF/help pass is not a proctor image or course pass. Existing deploy-check owns eight stages and serial dev/rehearsal E2E. Offline archive staging is separate.

## Exact Full Artifact Input

[offline-artifact-input.json](🎯️contracts/offline-artifact-input.json) supplies one coherent fresh tag and exact native argv arrays for actual test, deploy-check and bundle, with fresh ticket-generated output and expected roster. Bind workspaceRoot and actually authenticated native frontend on the Linux worker before execution; preserve original release mode. No historical private Source tag/input identity is reused.

Verify five files plus certificates directory, resolved same-tag proctor and local digest-tag Caddy, both never, full archive SHA/bytes, Docker save manifest/config/layer hashes and two unique image configurations (not necessarily two manifest entries because Caddy has aliases). Proctor config must match original actual revision/version/time and linux/amd64; the Caddy local tag name alone does not prove its content, so compare config identity against the digest-resolved source image evidence. No owned completed-bundle proof currently exists.

Existing docker-stack-check exercises the image-extracted online stack, not loading this archive or the staged offline YAML. A later independently authorized isolated-host load plus compose up --detach --wait is still required to claim offline runtime proof. Its operator command text is documentation, not an agent raw-Docker execution route. Current-tree nine-suite and extra typecheck acceptance remain separate; no publication is authorized.

## Files And Hashes

- 🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🚛️bundle/🧬️schema/🔣️.json — 803B; SHA256 ee5d649f333d5d98f0897248a7e711a89b6e87bf0e669a34800434a38635385e
- 🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🚛️bundle/🔣️.json — 1912B; SHA256 f815738c13d3a1350b4a8203507c8d19b84df59e69394fc7cc7acae68c66e380
- 🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️deploy/🌐️sealed-publication-and-bundle/🟦️.ts — 17698B; SHA256 c8eb87311b653c6e443898c5af0233461eafe1ea97d1fe55611c5a5d93edd4de
- 🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🎁️stack-bundle/🟦️.ts — 9392B; SHA256 97952fc4904739401bf9784feafca2c3057d06a96ec14bab130a8b38b45a4ee4
- 🎓️teaching/🏛️architecture/📚️course/README.md — 82127B; SHA256 4801e26c4444d89975eac4b443597ad9f1361888c9f7d6bf2199126df9b575ad
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/🎯️contracts/offline-stack.patch — 2498B; SHA256 f2016f571f33aad8f5aee444fa955f347f88354cf4c4508369d501ac52c5de3f
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/🎯️contracts/offline-artifact-input.json — 2976B; SHA256 2ddf99051b0e7a96aa02397ef860dfd7f4cbcb6b12e5d73b443ad1dd8d677c3a
