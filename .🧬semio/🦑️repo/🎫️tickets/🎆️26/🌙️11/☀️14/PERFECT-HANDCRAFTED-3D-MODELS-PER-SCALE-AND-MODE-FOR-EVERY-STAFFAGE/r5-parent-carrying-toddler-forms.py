"""🦴 Round-5 hand-stated form changes of `humans/parent-carrying-toddler` written into the asset's own solid scripts (`r3-parent-carrying-toddler-lists50.ts`, `r3-parent-carrying-toddler-s50.ts`, `r3-parent-carrying-toddler-coarse.ts`): every replaced statement is given in full, old text -> new text; the files are re-read right before writing and a missing old text is reported, not guessed.

Usage: python3 r5-parent-carrying-toddler-forms.py
"""
import os

T = os.path.dirname(os.path.abspath(__file__))

EDITS = {
    "r3-parent-carrying-toddler-lists50.ts": [
        ("[1.375, -0.008, 0.078, 0.112], [1.405, -0.013, 0.066, 0.096], [1.43, -0.018, 0.052, 0.074]];",
         "[1.375, -0.004, 0.074, 0.112], [1.405, 0.003, 0.064, 0.096], [1.43, 0.003, 0.058, 0.074]];\n/** 📐 The plane normal of a rib hoop in the upright figure: ribs fall about 24 degrees from their heads at the spine to the costal cartilages in front. */\nexport const RIB_NORMAL: V3 = [Math.sin((24 * Math.PI) / 180), Math.cos((24 * Math.PI) / 180), 0];"),
        ("m(revolve({ id: `rib-${10 - i}`, bone: \"torso\", ...BONE }, ring(rd, 0.006), T(x, y), [1, 0, 0], {",
         "m(revolve({ id: `rib-${10 - i}`, bone: \"torso\", ...BONE }, ring(rd, 0.006), T(x, y), D(\"torso\", RIB_NORMAL), {"),
        ("    m(ellipsoid({ id: `scapula-${n}`, bone: \"torso\", ...BONE }, [0.07, 0.007, 0.048], T(-0.088, 1.35, side * 0.102), compose(aboutW(4), aboutU(side * 28))), \"torso\");",
         "    m(loft({ id: `scapula-${n}`, bone: \"torso\", ...BONE }, [st(-0.07, 0, side * -0.022, 0.012, 0.007, 2), st(-0.035, 0, side * -0.01, 0.03, 0.007, 2.2), st(0.005, 0, 0, 0.045, 0.007, 2.4), st(0.045, 0, side * 0.008, 0.05, 0.007, 2.4), st(0.068, 0, side * 0.005, 0.036, 0.007, 2.2)], [true, true], { transform: { translation: T(-0.088, 1.35, side * 0.102), rotation: compose(aboutW(4), aboutU(side * 28)) } }), \"torso\");\n    m(capsule({ id: `scapular-spine-${n}`, bone: \"torso\", ...BONE }, T(-0.1, 1.4, side * 0.07), T(-0.03, 1.446, side * 0.165), 0.0065, 0.0075), \"torso\");"),
        ("    m(ellipsoid({ id: `ilium-${n}`, bone: \"pelvis\", ...BONE }, [0.062, 0.07, 0.012], P(-0.022, 0.985, side * 0.106), compose(aboutU(side * 32), aboutV(side * -22))), \"pelvis\");",
         "    m(loft({ id: `ilium-${n}`, bone: \"pelvis\", ...BONE }, [st(-0.06, 0, 0, 0.0085, 0.02, 2), st(-0.03, 0, 0, 0.0085, 0.036, 2.2), st(0.01, 0, 0, 0.0085, 0.058, 2.4), st(0.045, 0, 0, 0.0085, 0.07, 2.6), st(0.062, 0, 0, 0.0085, 0.06, 2.6), st(0.072, 0, 0, 0.0075, 0.034, 2.2)], [true, true], { transform: { translation: P(-0.022, 0.985, side * 0.106), rotation: compose(aboutU(side * 32), aboutV(side * -22)) } }), \"pelvis\");"),
        ("m(revolve({ id: `toddler-rib-${i + 1}`, bone: \"toddler-torso\", ...BONE }, ring(rd!, 0.0045), Tt(0.025, y!), [1, 0, 0], {",
         "m(revolve({ id: `toddler-rib-${i + 1}`, bone: \"toddler-torso\", ...BONE }, ring(rd!, 0.0045), Tt(0.025, y!), D(\"toddler-torso\", RIB_NORMAL), {"),
        ("import { ASSET, FRAMES, H, L, X, aboutU, aboutV, aboutW, boneDeg, capsule, compose, ellipsoid, loft, mirrored, part, revolve, st, thOf, toRight, tube, type V3 } from \"./r3-parent-carrying-toddler-kit.ts\";",
         "import { ASSET, D, FRAMES, H, L, X, aboutU, aboutV, aboutW, boneDeg, capsule, compose, ellipsoid, loft, mirrored, part, revolve, st, thOf, toRight, tube, type V3 } from \"./r3-parent-carrying-toddler-kit.ts\";"),
    ],
    "r3-parent-carrying-toddler-s50.ts": [
        ("const hair: Row[] = [[1.57, -0.112, -0.06, 0.056, 2], [1.6, -0.128, -0.04, 0.069, 2], [1.63, -0.134, -0.03, 0.075, 2], [1.66, -0.132, -0.04, 0.074, 2]];",
         "const hair: Row[] = [[1.575, -0.112, -0.03, 0.058, 2.1], [1.6, -0.127, 0.0, 0.07, 2.1], [1.63, -0.132, 0.02, 0.074, 2.1], [1.66, -0.134, 0.03, 0.076, 2]];"),
    ],
    "r3-parent-carrying-toddler-coarse.ts": [
        ('    out.push(loft(glove("glove-r", "hand-r"), [st(-0.012, 0.004, 0, 0.03, 0.019), st(0.05, 0.009, 0, 0.042, 0.016), st(0.1, 0.009, 0, 0.042, 0.016), ...(kk ? [st(0.14, 0.012, 0, 0.04, 0.012), st(0.16, 0.016, 0, 0.034, 0.009)] : [st(0.155, 0.014, 0, 0.038, 0.015)])]));',
         '    out.push(loft(glove("glove-r", "hand-r"), kk ? [st(-0.012, 0.004, 0, 0.03, 0.019), st(0.05, 0.009, 0, 0.042, 0.016), st(0.1, 0.009, 0, 0.042, 0.016), st(0.14, 0.012, 0, 0.04, 0.012), st(0.16, 0.016, 0, 0.034, 0.009)] : [st(-0.012, -0.001, 0, 0.033, 0.025), st(0.05, 0.003, 0, 0.044, 0.024), st(0.1, 0.004, 0, 0.044, 0.024), st(0.155, 0.009, 0, 0.039, 0.021)]));'),
    ],
}


def apply(edits):
    for name, pairs in edits.items():
        path = os.path.join(T, name)
        text = open(path, encoding="utf-8").read()
        for old, new in pairs:
            if new in text:
                continue
            if text.count(old) != 1:
                print("[DEBUG] %s: old text found %d times: %s" % (name, text.count(old), old[:90]))
                continue
            text = text.replace(old, new)
        open(path, "w", encoding="utf-8").write(text)
        print("written", name)


apply(EDITS)
