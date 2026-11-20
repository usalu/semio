/** 🦴️ Prints the rest and default-pose bone frames of the draft standing adult as JSON (for the Python modelling script). */
import { frames, poseState } from "./volumes-lib.ts";
import { draftAsset } from "./r3-standing-adult-view.ts";

const a = draftAsset(process.argv[2]);
process.stdout.write(JSON.stringify({ rest: frames(a), pose: poseState(a, 50).frames }));
