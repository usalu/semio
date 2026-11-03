import atexit, json, os, sys

LOG = {}


def install():
    import manim
    from manim import Scene

    original = Scene.render

    def render(self, *args, **kwargs):
        for module in list(sys.modules.values()):
            cls = getattr(module, "_Motion", None)
            if cls is not None and getattr(cls, "advance", None) is not None and not getattr(cls, "_probed", False):
                cls._probed = True
                advance = cls.advance

                def probed(motion, dt, advance=advance):
                    key = motion.content
                    entry = LOG.setdefault(key, {"n": 0, "firstFrame": None, "wall": 0.0, "dur": len(motion.frames) / motion.fps, "fps": motion.fps, "loops": motion.loops, "ids": set()})
                    entry["ids"].add(id(motion))
                    entry["wall"] += dt
                    entry["n"] += 1
                    advance(motion, dt)
                    entry["elapsed"] = motion.elapsed
                    entry["rate"] = motion.rate
                    entry["frame"] = motion.frame
                    entry["count"] = len(motion.frames)

                cls.advance = probed
        return original(self, *args, **kwargs)

    Scene.render = render


def dump():
    out = os.environ["PROBE_OUT"]
    rows = []
    for key, e in LOG.items():
        e = dict(e)
        e["ids"] = len(e["ids"])
        e["cast"] = key
        rows.append(e)
    with open(out, "w", encoding="utf8") as handle:
        json.dump(rows, handle, indent=1)


if __name__ == "__main__":
    install()
    atexit.register(dump)
    from manim.__main__ import main
    sys.argv = ["manim"] + sys.argv[1:]
    main()
