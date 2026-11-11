# Bounded Media Inspection Through The Tool Registry

repo tools help and list confirm a registered ffmpeg tool with frame-extract capability, a system pin and the SEMIO_TOOL_FFMPEG override. PATH did not supply it, but the existing repo-managed render Python environment already contains imageio_ffmpeg/binaries/ffmpeg-win-x86_64-v7.1.exe. Setting that override for one command and invoking repo tools exec --light ffmpeg -- -version passed. No program was spawned outside the registry and no package or manifest was changed.

For this Windows session use the absolute bundled binary under C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Lib/site-packages/imageio_ffmpeg/binaries. A bounded frame query uses -hide_banner -v error -ss seconds -threads 1 -i current_partial -frames:v 1 -threads 1 -update 1 -y ticket_generated_png. Inspect at most a few already-rendered partials per query and verify their current-attempt timestamps first. This avoids importing PyAV through UV and its 6 GB dependency admission for a single frame.

The light mode is limited to short read-only diagnostics. Complete decode, video encoding, geometry, tests and other expensive operations retain normal capacity admission. The temporary override is local to the tool command; repository runtime configuration and source code remain unchanged.
