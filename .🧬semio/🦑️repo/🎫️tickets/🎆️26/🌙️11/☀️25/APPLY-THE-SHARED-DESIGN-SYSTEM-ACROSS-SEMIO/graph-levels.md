# Workspace stylesheet exports in the build graph

The TypeScript module resolver reads the declaring package but leaves a direct stylesheet export unresolved. The command graph now uses the observed resolution host to admit a physically present stylesheet named by an exact, literal public export. Both root and subpath exports are covered. The existing registry-package exclusion, generated output handling, declaration resolution and compiled-output ownership checks still apply. Missing files still report the declared destination. Directory targets and package-path escapes remain unresolved. The command walk records the stylesheet as a resource.

Repo test-file reproduced the stylesheet failure before the fix, then passed all five tests across package metadata edits, unresolved workspace imports and producer paths. The regression compares the resolved stylesheet path with the runtime's native package resolver, checks that the command closure contains the resource, and rejects traversal, directories and unowned compiled outputs.

Changed paths within 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs

- 🟨️.mjs
- 🧪️tests/package-metadata-edits.test.ts


The source resolver also resolved the actual presentation deck Storybook stylesheet to its existing package CSS entry. The already running repository service continued returning its earlier unresolved-export result on retry. It was left running for the other agents. The presentation tests ran directly against the working tree. The observation schema had two definitions with the same key for the portable parser owner. It now retains the single active definition.
