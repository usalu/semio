import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2];
let t = readFileSync(file, "utf8");
const swap = (a, b) => { if (!t.includes(a)) throw new Error("missing: " + a.slice(0, 70)); t = t.replace(a, () => b); };
swap(`            motion = self.build(root)
            for step in CASE["steps"]:
                motion.rate = step["rate"]
                motion.advance(step["advance"])`, `            motion = self.build(root)
            S.SCENE_CLOCK.now = 0.0
            motion.sync()
            for step in CASE["steps"]:
                motion.rate = step["rate"]
                S.SCENE_CLOCK.now += step["advance"]
                motion.sync()`);
swap(`            motion = self.build(root)
            motion.update(0.12)
            self.assertEqual(motion.frame, 1)`, `            motion = self.build(root)
            S.SCENE_CLOCK.now = 5.0
            motion.update(0.0)
            S.SCENE_CLOCK.now = 5.12
            motion.update(0.0)
            self.assertEqual(motion.frame, 1)`);
swap(`                    motion = S.staffage_motion("case", unit=1.0, rate=case["rate"])
                    motion.update(case["advance"])`, `                    motion = S.staffage_motion("case", unit=1.0, rate=case["rate"])
                    S.SCENE_CLOCK.now = 3.0
                    motion.update(0.0)
                    S.SCENE_CLOCK.now = 3.0 + case["advance"]
                    motion.update(0.0)`);
swap(`class Parser(unittest.TestCase):`, `class Playback(unittest.TestCase):
    VECTORS = json.loads(Path(__file__).parents[5].joinpath("🧍️staffage", "🧫️fixtures", "🎬️playback-clock", "🔣️.json").read_text(encoding="utf-8"))

    def test_the_shared_vectors_give_the_authored_frames(self):
        for row in self.VECTORS["playback"]:
            for sample in row["samples"]:
                frame = S.playback_frame(sample["t"], row["fps"], row["count"], row["durationS"], row["loop"])
                self.assertEqual(frame, sample["frame"], (row["name"], sample))

    def test_an_accumulating_clock_and_a_seek_agree_on_every_frame(self):
        for row in self.VECTORS["playback"]:
            clock = 0.0
            for i in range(400):
                stepped = S.playback_frame(clock, row["fps"], row["count"], row["durationS"], row["loop"])
                seeked = S.playback_frame(i / 24, row["fps"], row["count"], row["durationS"], row["loop"])
                self.assertEqual(stepped, seeked, (row["name"], i))
                clock += 1 / 24

    def test_a_motion_shows_the_exported_frame_n_at_n_over_fps_and_wraps_without_a_seam(self):
        with tempfile.TemporaryDirectory() as root:
            motion = S._Motion(synthetic(Path(root)), 1.0, [0, 0, 0], 1.0, None)
            S.SCENE_CLOCK.now = 7.0
            motion.sync()
            for n in range(9):
                S.SCENE_CLOCK.now = 7.0 + n / motion.fps
                motion.sync()
                self.assertEqual(motion.frame, n % 3)
            S.SCENE_CLOCK.now = 7.0
            before = Motion.line_y(None, motion)
            motion.sync()
            self.assertEqual(motion.frame, 0)
            self.assertAlmostEqual(Motion.line_y(None, motion), CASE["expected"]["lineY"][0], places=6)
            self.assertNotEqual(before, None)

    def test_seeking_equals_playing_the_same_scene_time(self):
        with tempfile.TemporaryDirectory() as root:
            folder = synthetic(Path(root))
            played = S._Motion(folder, 1.0, [0, 0, 0], 1.0, None)
            seeked = S._Motion(folder, 1.0, [0, 0, 0], 1.0, None)
            S.SCENE_CLOCK.now = 2.0
            played.sync()
            for i in range(1, 40):
                S.SCENE_CLOCK.now = 2.0 + i / 15
                played.sync()
                seeked.seek(i / 15)
                self.assertEqual(played.frame, seeked.frame, i)
                self.assertEqual([slot.get_points().tolist() for slot in played.slots], [slot.get_points().tolist() for slot in seeked.slots], i)

    def test_a_removed_motion_keeps_its_phase_and_restart_begins_again(self):
        with tempfile.TemporaryDirectory() as root:
            motion = S._Motion(synthetic(Path(root)), 1.0, [0, 0, 0], 1.0, None)
            S.SCENE_CLOCK.now = 1.0
            motion.sync()
            S.SCENE_CLOCK.now = 1.1
            motion.sync()
            self.assertEqual(motion.frame, 1)
            S.SCENE_CLOCK.now = 9.1
            motion.sync()
            self.assertEqual(motion.frame, 1)
            motion.restart()
            S.SCENE_CLOCK.now = 9.2
            motion.sync()
            self.assertEqual(motion.frame, 1)
            self.assertAlmostEqual(motion.elapsed, 0.1, places=9)


class Parser(unittest.TestCase):`);
writeFileSync(file, t);
