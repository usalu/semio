# Attempt 51 Startup Route Audit

Read-only HTTP inspection of `http://127.0.0.1:6080/?plugin=note&role=editor` returned the Note-specific `✏️s/🧑‍💻dev/🎭️variants/🗒️note/🚀️entry/🟦️.ts`. That entry requires `VITE_SEMIO_PLUGIN=note`, composes the specific catalog, and calls `bootFrameworkOsDev` with that variant. The live `virtual:semio-playground-session` response names variant `note`, plugin `note`, and default editor app `s.note.note@1/*#editor`. React’s `?plugin=` is not its selection mechanism: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔗️boot-query/🟦️.ts` lines 8–11 says the server selects the variant; the `role` query is consumed. No missing URL flag was identified.

The generic `🧑‍💻dev/🚀️entry/🟦️.ts` does boot an empty catalog, but it is not the entry served on this port; an earlier preliminary message incorrectly assumed it was.

React readiness in `📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🖼️canvas-footer/🟠️footer-items/🟦️.tsx` lines 58–90 sets ready when `session && initialExampleReady`, sets an error beacon on `error`, and has a separate not-found beacon. Shell chrome with neither ready nor error is consistent with a pending session or initial-example readiness; also inspect the not-found attribute. This source inspection does not determine the underlying stalled operation.
