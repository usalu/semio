# Process Extension Activation Audit

The generated development plugin inventory at ✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🔌️plugins.json includes the process plugin and four declarative extensions: concrete, metal, robotic, and wood. Each extends and depends on process and contributes to process.machines. The generated playground catalog selects s.process.process3d@1/*#editor, while extension contribution payloads target appId process3d-play. This identity difference requires an actual host activation and contribution-delivery test; static registration alone does not establish that the editor receives machines.

The React entry ✏️s/🧑‍💻dev/🚀️entry/⚛️react/🟦️.ts passes installed catalog, plugins, and surface session factories to framework boot. The editor exposes setContributions. The extension catalog test ✏️s/🔌️plugins/🏭️process/🧩️extensions/🧪️tests/🔬️catalog-json/🦀️.rs checks serialized machine rows against neutral fixtures and serde_json, but does not prove activation or delivery to the mounted artifact editor.

Verification targets: launch the actual process3d artifact editor through the React host; confirm all four extension packages are installed and activated, and that their payloads reach the editor despite the app identity difference; inspect the machine catalogue and add a machine through the UI; exercise an edit and verify resulting document state. Confirm disabling or omitting an extension removes its machines. Refresh generated inventory through the repository catalog generation target if inputs change.

This is a source audit; runtime activation has not been verified.
