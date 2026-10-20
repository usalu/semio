// 🧾 Spec builder of the 11-directory shape fixes (data only). usage: bun emoji-spec.mjs > emoji-spec.json
const F = "🧰️framework";
const E = `${F}/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`;
const R = `${F}/🛍️products/🦑️repo/🔨️modules/📚️library`;
const LO = `${F}/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview`;
const SH = `${E}/🏛️ShellHost`;
const L = `${R}/⚖️laws/🎯️tool-job-coverage/🔁️live-replay`;
const M = `${R}/⚖️laws/🔬️interactivity-audit/🖥️mounted`;
const T = `${R}/🧪️tests/🔄️transaction-v2`;
const s = (line, from, to) => ({line, from, to});
const files = [
  {file: `${LO}/👆️swipe/🟦️.ts`, head: "The swipe phase of LayeredOverview: the touch drive, the scroll and pointer-capture helpers and the swipe hook.", set: [s(4, "👆", "🕹"), s(20, "👆", "🖐"), s(77, "👆", "↔")]},
  {file: `${LO}/📋️types/🟦️.ts`, head: "The types of LayeredOverview: modes, panes, cards, chrome, labels, insets and props.", set: [s(11, "🥞", "🎴"), s(25, "🥞", "📄"), s(80, "📱", "📍")]},
  {file: `${LO}/🧰️pane-helpers/🟦️.ts`, head: "The pane helpers of LayeredOverview: poster capture, reduced motion, cells, hash sync, veil and window helpers.", set: [s(67, "🔗", "🪞")]},
  {file: `${LO}/🧵️pane/🟦️.tsx`, head: "The pane of LayeredOverview: mounting, error boundary, placeholder and the card host."},
  {file: `${LO}/💡️hints/🟦️.tsx`, head: "The swipe hints of LayeredOverview: the neighbouring page and its direction at each edge of the view.", set: [s(41, "🧭", "🧩")]},
  {file: `${E}/🌐️World3dHost/🧭️fit-and-presence/🔸️meshes-ref/🟦️.ts`, set: [s(183, "🏁", "📍")]},
  {file: `${SH}/🎥️tutorial-orchestration/🔶️scope/🟦️.ts`, set: [s(86, "⏱", "⏯")]},
  {file: `${SH}/📤️document-transfer-tabs/🔶️scope/🟦️.tsx`, set: [s(60, "📤", "🎯")]},
  {file: `${SH}/📬️artifact-opening/🔶️catalog/🟦️.ts`, set: [s(52, "👁✏", "🔤"), s(161, "⚖", "💾"), s(178, "⚖", "🤝")]},
  {file: `${SH}/📬️artifact-opening/🔸️open-artifact-app/🟦️.ts`, set: [s(111, "👁✏", "🗂")]},
  {file: `${SH}/📬️artifact-opening/🟠️open-entries/🟦️.ts`, set: [s(40, "👁✏", "🗃"), s(73, "🧯", "🔒"), s(83, "⚖", "🍞"), s(97, "⚖", "🌉")]},
  {file: `${SH}/🗄️shell-uri-documents/🟠️close-document/🟦️.ts`, set: [s(114, "🔗", "🧷"), s(185, "📎", "📂"), s(191, "📎", "🔁")]},
  {file: `${SH}/🛂️directory-route/🟠️shell-uri/🟦️.ts`, set: [s(79, "🧭", "🚦")]},
  {file: `${F}/🛍️products/📚️course/🧪️tests/🏠️home-grid/🟣️shared/🟦️.tsx`, set: [s(41, "🎨", "📚")]},
  {file: `${L}/🧊️retained-ownership/🟦️.ts`, head: "The cold layout relay and universal retained-ownership contract of the live replay law."},
  {file: `${L}/🧮️fem-numerical-microcursor/🟦️.ts`, head: "The FEM numerical micro-cursor contract of the live replay law.", set: [s(3, "🧮", "🔥")]},
  {file: `${L}/🪟️fem2d-mounted-session/🟦️.ts`, head: "The mounted FEM2D session contract of the live replay law."},
  {file: `${M}/🎞️frame-transaction/🟦️.ts`, head: "The frame transaction audit of the mounted interactivity law."},
  {file: `${M}/🧭️checked-rust-block-scope/🟦️.ts`, set: [s(5, "🧭", "🔎")]},
  {file: `${T}/🏗️fixture-builders/🟦️.ts`, head: "The fixture builders of the transaction-v2 suite: run roots, plans, snapshots and fixture variants."},
  {file: `${T}/💥️child-control/🟦️.ts`, head: "The child-process control of the transaction-v2 suite: the active reference fixture, spawned children and crash drivers."},
  {file: `${T}/📜️boundary-contracts/🟦️.ts`, head: "The boundary contracts of the transaction-v2 suite: fixture and snapshot types, goldens and phase tables."},
];
console.log(JSON.stringify({files}, null, 1));
