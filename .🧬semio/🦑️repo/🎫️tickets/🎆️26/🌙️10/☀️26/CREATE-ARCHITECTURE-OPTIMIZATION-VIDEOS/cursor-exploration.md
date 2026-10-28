# Cursor Preview Exploration

Root AGENTS.md read; implementation unchanged.

Canonical preview UI: 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🟦️.tsx. Uses actual shared LayeredOverview and OverviewCardOpenChip. German Ansehen label is schema-owned presentation/🔣️.json.

Production motion: presentation/🥞️hierarchy/🖌️render/🐍️.py choreography(), lines 76–134. Moves to topic chip, reveals and clicks, then video chip, reveals and clicks, then parks. Timing presentation/🔣️.json lines 49–57: .4 topic travel, .6 reveal, .3 press, .1 hold, .5 video travel, .6 reveal, .3 press, .2 final travel: 3 seconds total. Smoothstep easing; near-coincident chips insert 96×64 detour at lines 99–101.

Explicit wandering demo: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/ENERGY-CLIP-STYLE/render-layered-navigation.py. It authors extensive cross-grid travel and hovers both topic/video Ansehen. Renders 12fps, 1600×900 via browser.render_batch, encodes through runtime.run_command(ffmpeg_command()), then copies layered-navigation.mp4 beside script. Best candidate for replacing a standalone preview with slow approach to one measured chip, hold, retreat, no clicks.

Tests: presentation/🖱️interaction/🧪️tests/🐍️.py with adjacent portable 🔣️.cases.json verifies native hover and choreography. Around line 581 explicitly asserts 3-second focus and restore, requiring expectation changes if production timing changes. Browser and hierarchy sibling test suites cover native layers. Registered full suite: repo ./🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts test. Managed Python 3.12 uv runtime, third-party native/image oracles. Rendering uses same script render with explicit language/audience.

Recommendation: distinguish standalone preview from committed lesson navigation. Preserve two-click lesson flow unless explicitly changing it. Define portable single-target preview contract: fixed target ID, >=2-second approach, >=2-second stable hold, no clicks or other Ansehen hover. Validate actual per-frame browser hit paths and independent Pillow rectangle containment. Keep camera stable during hold; otherwise parallax moves geometry beneath cursor. Reuse existing production browser/runtime interfaces and store generated outputs under active ticket generated folder.
