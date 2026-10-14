# Artifact Inspector Ownership

The current Note mutation inspector is physically owned by the plugin at `✏️s/🔌️plugins/🗒️note/🏭️bridge`, while its Cargo manifest requires the Note artifact package at `../🗿️artifacts/🗒️note/📦️packages/🦀️rust`. Deleting that artifact leaves this plugin-owned executable with a broken required dependency. This is a concrete ownership violation; no deletion or Cargo acceptance has been run for it yet.

The executable reads the actual Note dispatch aggregate descriptors and emits the runtime mutation inventory. Its descriptor rows and subset coordinates are production evidence and must survive any ownership change. An empty inventory, optional skipped executable, or copied fixture would not establish deletion safety.

The current script invokes synchronous Cargo without the original Script control, progress, or a finite child grant. Moving the executable alone would leave this interface defect unresolved. The final artifact-owned command needs explicit original authority, bounded output, cancellation, and a real receiving command registered in both GUI configurations.

The repository already has a closed mutation-inventory provider declaration at `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏭️inventory/🔌️providers/🧬️schema/🔣️.json`. Discovery reads owner-authored Cargo workspace metadata, confines scripts and roots to their defining owner, refuses symlinks and ambiguous ownership, and admits only present roots. Note's artifact workspace currently declares no such provider. The older ancestor bridge convention also remains in the inventory orchestration and needs its own consumer audit before retirement.

The next complete cut should move the actual inspector into the Note artifact owner, author that owner's provider metadata, hand-port every real reference and Cargo path, and preserve all original mutation inventory assertions. It must verify the real production output against the existing independent inventory oracle and exercise the retained plugin scope after removing the artifact in the language-neutral deletion model. No source files were copied to this ticket, and this report does not claim those changes are implemented.
