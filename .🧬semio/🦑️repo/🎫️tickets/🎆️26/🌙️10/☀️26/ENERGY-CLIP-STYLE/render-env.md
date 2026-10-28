# Physical Fundamentals Render Environment

The existing shared native interpreter is `C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe`. Read-only validation confirms Python 3.12.11, Manim 0.22.0, NumPy 2.5.4, imageio-ffmpeg 0.6.0 and PyAV 19.0.2. Native Manim imports successfully. The packaged FFmpeg binary is present in its imageio-ffmpeg distribution. No environment provisioning or shared-environment changes are required.

The authoring dependency project remains `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python`, with the same locked runtime versions. Only `physical_fundamentals` is selected: source ordinals 1–9 contain its intro and eight physics beats. The manifest includes one assembled physics chapter.

The render uses the existing CLI through `repo capacity run`, with 640×360 output, 6 fps, four workers, and a 16384 MB capacity grant. `--log-output` creates immutable `attempt-<time_ns>.log` files inside each owned segment directory. All generated output, font cache settings and reports remain below this ticket's `🗑️generated` root. The shared interpreter is reused read-only with bytecode generation disabled.

The source narration resolver finds zero existing canonical clause recordings for all nine physical sources. The render pipeline will produce visual/subtitle video without generating speech.
