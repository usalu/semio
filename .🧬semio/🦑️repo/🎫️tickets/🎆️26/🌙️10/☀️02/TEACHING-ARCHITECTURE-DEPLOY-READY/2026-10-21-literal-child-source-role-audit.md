# Literal Native Child Source Role Audit

Static actual-code audit; no parser/helper/test invoked. Corrects the earlier report-only assertion that runtimeSourceEntries metadata must be added.

Actual caching/📥️inference/🟨️.mjs commandUrlReferences66–74 parses literal new URL whose second argument recursively mentions import.meta.url. Thus both parent presence ./👷️child/📜️script.ts and closure ./👷️read.mjs (even through physicalSourceOriginV1) are discovered. build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs workspaceUrlTargetV169–75 identifies .ts and .mjs as Source; relativeScriptInputs265–270 recursively visits the URL target, then static relative imports as Source and JSON as resources. Presence child script imports sibling implementation, which imports real owner dependencies. Neither current literal needs metadata.semio.runtimeSourceEntries. That field at targetScriptSources321 is an optional explicit owner-root mechanism for otherwise undiscovered runtime Source, not a ticket control schema field. An explicit resource override or generated-output declaration could change role; no such override is proposed for these children.

The actual GC role test reads parser/script-inputs/runtime text, registers actual observed parser providers, and checks modules/compiledNames versus resources plus transitive generation/schema membership. This is an authored runtime witness, unexecuted. Presence child currently has static derivation by inspection; an analogous actual role assertion can verify it without adding metadata.

Existing Source81 template already explicitly witnesses parser.mjs, script-inputs.mjs, component roster, data.mjs and exact-read policy JSON. New direct explicit witnesses need parent presence test/corpus/schema, static child script+implementation, closure runtime/worker/schema/corpus/test, and changed fixture/reuse/storage owners as Root integrates. Worker direct imports additionally require these absent historical witnesses:

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧭️file-owner/🟨️.mjs (generation static transitive import; no further imports)
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📥️authority.json

Worker also statically imports already-witnessed data.mjs and exact-read policy JSON, and new local closure schema. Ordinary first-party imports elsewhere remain governed by genuine Source capture; this is not a claim every transitive package import has been independently inventoried for explicit roster membership. Upcoming multipart/block additions require their own final direct-read list.

PhysicalSourceOrigin controls locating physical reader files, while the presence child artifact-relative literal locates its compiled child. Native actual execution is still required to verify emission, receiver ingress and child completion; Source-role static derivation alone is not a test pass or loaded-byte proof.
