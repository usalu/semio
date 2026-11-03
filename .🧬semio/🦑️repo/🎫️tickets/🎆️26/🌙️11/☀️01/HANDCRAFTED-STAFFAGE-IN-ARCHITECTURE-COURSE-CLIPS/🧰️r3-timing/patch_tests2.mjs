import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2];
let t = readFileSync(file, "utf8");
const swap = (a, b) => { if (!t.includes(a)) throw new Error("missing: " + a.slice(0, 70)); t = t.replace(a, () => b); };
swap(`            S.SCENE_CLOCK.now = 7.0
            before = Motion.line_y(None, motion)
            motion.sync()
            self.assertEqual(motion.frame, 0)
            self.assertAlmostEqual(Motion.line_y(None, motion), CASE["expected"]["lineY"][0], places=6)
            self.assertNotEqual(before, None)
`, `            S.SCENE_CLOCK.now = 7.0 + 3 / motion.fps
            motion.sync()
            self.assertEqual(motion.frame, 0)
            _, boxes = pieces(VGroup(*motion.slots))
            self.assertAlmostEqual(min(box[0][1] for box in boxes if box[1][1] - box[0][1] < 1e-6), CASE["expected"]["lineY"][0], places=6)
`);
swap(`    def test_a_removed_motion_keeps_its_phase`, `    def test_a_real_scene_keeps_the_motion_on_video_time_through_waits_plays_and_animated_groups(self):
        from manim import FadeIn, Scene, tempconfig

        with tempfile.TemporaryDirectory() as root:
            folder = synthetic(Path(root))
            seen = []

            class Probe(Scene):
                def construct(scene):
                    motion = S._Motion(folder, 1.0, [0, 0, 0], 1.0, None)
                    scene.add(motion)
                    for run in (1.0, 0.33, 0.7):
                        scene.wait(run)
                        seen.append((float(scene.renderer.time), motion.elapsed))
                    scene.play(FadeIn(motion), run_time=0.41)
                    seen.append((float(scene.renderer.time), motion.elapsed))
                    scene.wait(0.5)
                    seen.append((float(scene.renderer.time), motion.elapsed))

            with tempconfig({"frame_rate": 15, "write_to_movie": False, "dry_run": True, "disable_caching": True, "verbosity": "ERROR", "progress_bar": "none"}):
                Probe().render()
            for video, cast in seen:
                self.assertAlmostEqual(cast, video - 1 / 15, places=9, msg=seen)

    def test_a_removed_motion_keeps_its_phase`);
writeFileSync(file, t);
