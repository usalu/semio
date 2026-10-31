# Native Clip Execution Plan R92

The registered entry is `repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render`. It invokes the authoring Python 3.12 environment through uv; its pinned direct dependencies are Manim 0.22.0, NumPy 2.5.4 and imageio-ffmpeg 0.6.0. Test dependencies include fonttools, SciPy and Shapely. No dependency change is proposed.

Run native tests selectively through a retained ticket harness via `repo tools exec --light python` only if that interface can select the teaching Python environment; otherwise register/use the existing authoring command. The existing authoring `test` command selects many unrelated suites and is unsuitable for these three final gates. Exact selected identities are BimBrowserMetadataContract.test_source_discovery_skips_browser_runtime, BimClipContract.test_all_bilingual_clauses_fit_native_caption_layout_for_both_audiences and BimClipContract.test_each_chapter_has_unique_bilingual_narration_and_discoverable_scene in 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️.

Render each language with all six repeated `--video` selections (bim_information,bim_objects,bim_representations,bim_models,bim_exchange,bim_consistency); omit audience to render both speaker and spectator, giving 24 complete outputs. Use `--quality l --fps 4 --resolution 480,270 --jobs 1 --no-play --log-output --output-root` under the ticket generated directory for first complete previews. The finite jobs bound avoids runaway concurrent geometry work. Native lifecycle probe supports snapshot images per clause but currently only spectator; full movies cover both. Full HD output can follow after previews pass.

The existing `BimPresentationContract.test_native_lifecycle` executes actual Manim animation setup/interpolation/finish/cleanup and Cairo capture, saving each clause and final frame in English/German. It patches play/wait to avoid encoding holds and does not replace the required complete native movie renders. Its selected topic filter is SEMIO_TEST_BIM_TOPICS.

All commands await launcher repair. No command bypass, launcher edit, Git mutation or worktree used.
