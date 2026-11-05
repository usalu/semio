# Shooting Presence Producer

The existing editor presence schema is authoritative: selectedShotIds and camera are shared ephemeral state. Config mutations continue to own the local camera and shot selection. Neither presence delivery nor camera navigation edits the shared shooting document.

## Implementation

The registered producer now constructs ShootingCommandWork instead of a reducer that can only return Complete. The work dispatches the existing command reducer and publishes CompleteWithEphemeral for emitted SetCamera or SetShotSelection config mutations. It preserves the other field from captured local presence. This also covers loading saved cameras and selection of newly added shots. Unrelated commands retain Complete and avoid cloning the presence root.

The work owns only a static tool id and a consumed flag. Its terminal frame release is declared, it uses the existing bounded work extent and original producer admission, and it owns no heap roots requiring a new retirement implementation. The editor now supplies the bounded presence one-item preparation factory, required because the framework trait defaults to None and refuses Presence publication without it. The existing presence store and peer/root retirement factories retain lifecycle authority.

Presence publication authority is added to setCamera, loadSavedCamera, setShotSelection and addShot. All 35 production publication contracts were compared with the language-neutral retained command fixture using the repository tool ingress and Python JSON as an independent parser; all agree.

## Regression Coverage

The language-neutral presence fixture supplies selected shot ids and camera position. The mounted editor regression dispatches selection, camera, then cleared selection; compares real local presence with the serde_json oracle; verifies a nonzero published presence generation and nonempty pack; and verifies the shared document does not change. It then invokes the framework presence round trip law through the real peer roster ingress and departure, including typed peer root retirement and unchanged peer document.

A second mounted regression covers saved-camera loading and newly added shot selection, exercising the remaining two publication authorities and preserving selection across camera loading.

The initial regression was authored before the implementation. Native compilation and execution are delegated to the native owner already running the declared shooting Rust test target. No passing native result is claimed here until that execution completes.
