# Host resume after the read of the lane scripts

The fixture `reuse-marketplaces.xlsx` is on disk. `.cache/b/p/m` was empty, so publish could not run even after wasm. The Space wasm build (`space-cargo.log`) exited 101 on `semio-s-artifact-space-space-schema`: `decode_sqlite_snapshot_native` was compiled with 3 parameters while the trait takes 4. The file on disk now has the 4-parameter form and forwards `output`, matching the other artifacts. That compile used the older text. Rebuild Space only after the pane cargo releases the shared wasm target lock.

## Corrected order

Wasm still has to finish under `T/🗑️generated/claude-pane/native/target/wasm32-wasip2/wasm-prototype/`.

Pane, after `claude-host-prepare.ts` and `claude-pane-engines-publish.ts`:

- `claude-fulls-session.ts stdio-xlsx stdio,stdio-office`
- `claude-pane-publish.ts` reads the generated catalog, then `stdio` and `stdio-office` wasm paths
- `claude-pane-activate.ts`
- vite on 127.0.0.1:6272 with `SEMIO_PLUGIN=stdio-xlsx` and `✏️s/🧑‍💻dev/🎭️variants/📕️stdio-xlsx/🏗️builder/🌐️vite/🟦️.ts`
- `standalone-pane-journey.ts` from the repo root. Chromium now launches with `--js-flags=--experimental-wasm-jspi`.

Full S:

- `claude-fulls-session.ts` (variant `s`)
- `claude-fulls-publish.ts` with `plugin=wasm` pairs. Toolchain comes from `rustc -vV` when the layout receipt is absent.
- `claude-fulls-activate.ts s space stdio stdio-office`
- `claude-fulls-serve.sh s 6070`
- `claude-fulls-journey.ts` twice, `en` then `de`

`pack-typst-font-assets` from `.cache/b/r/m/published/632a4c57d2304c4d2912/prototype` wrote 8757072 bytes. Copies are the seed `infinite/.../dist/fonts/🔤️guestslim-typst-fonts.bin` and the publication `.cache/b/p/m/infinite/prototype/fonts/`. Sessions for `stdio-xlsx` and `s` are written. Engine packages are republished. `claude-pane-resume.py` waits for pane cargo pid 62591, then publishes, activates, serves 6272 and runs the journey. `claude-fulls-resume.py` waits for that resume and for the Space cargo, then publishes all three components, activates variant `s`, serves 6070 and runs the English and German journeys. Space cargo is blocked on that same wasm target lock; the snapshot file on disk already has the 4-argument decode and balances.
