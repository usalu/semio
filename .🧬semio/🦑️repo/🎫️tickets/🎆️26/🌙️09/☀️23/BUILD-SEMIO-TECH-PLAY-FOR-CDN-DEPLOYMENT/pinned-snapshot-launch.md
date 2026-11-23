# Pinned Snapshot Launch

Clone: `/Users/ueli/.sinc/play` (no .git, own .cache, ~5.4 GB before builds). Launcher copy at the same relative ticket path inside the clone; `native-play.sh` line 5 honours `SEMIO_WORKSPACE_ROOT`.

Bootstrap note: `repo incremental prepare` fails in the clone (`tools setup all` runs before `dependencies sync`, `@semio-tech/process` unresolved). Worked around by running `repo dependencies sync --all` then `repo tools setup all` inside the clone with the scrubbed env below. Do not re-run `prepare --fresh`.

Env helper: `U=$(env | grep -o '^SEMIO_[A-Z_]*=' | sed 's/=//;s/^/-u /')` (unsets every inherited SEMIO_*).

```
P=/Users/ueli/.sinc/play
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
cd $P && env -u REPO_ROOT -u SEMIO_MACHINE_STATE_DIRECTORY -u CARGO_TARGET_DIR -u CARGO_BUILD_BUILD_DIR -u NX_WORKSPACE_ROOT $U \
  SEMIO_LANGUAGE=en SEMIO_WORKSPACE_ROOT=$P SEMIO_BUILD_SERVICE=off BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 SEMIO_REPO_SEAL=live \
  SEMIO_BUILD_MODE=release NATIVE_PLAY_OUT=/Users/ueli/Documents/semio/.🧬semio/🌐hub/play-cdn-1122/pinned \
  bash "$P/$T/native-play.sh" site     # then: verify
```
(Detach it with the ticket's daemonize.py / native-chain.sh as usual; native-chain.sh was copied too.)

Proven: `native-play.sh step registry-generate "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry" repo ./📜️script.ts generate` -> `[ok] registry-generate` in 2 s from the clone (output "0 plugin crates, 0 playgrounds, 0 framework packages", written under the clone). Source-boundary preflight on the clone: rc 0, 0 OVER.
