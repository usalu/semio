"""🏗 Round 5, furniture/cafe-chair: the round-3 candidate builder fed with the round-5 body (`r5-cafe-chair-body.py`); writes the same candidate document and volumes source for `r3-cafe-chair-apply.py`.

Usage: python3 r5-cafe-chair-build.py
@see ./r3-cafe-chair-build.py
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def load(path, name):
    spec = importlib.util.spec_from_file_location(name, os.path.join(HERE, path))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


BUILD = load("r3-cafe-chair-build.py", "r3_cafe_chair_build")
_r3_module = BUILD.module


def module(name):
    return load("r5-cafe-chair-body.py", "r5_cafe_chair_body") if name == "body" else _r3_module(name)


BUILD.module = module

if __name__ == "__main__":
    BUILD.main()
