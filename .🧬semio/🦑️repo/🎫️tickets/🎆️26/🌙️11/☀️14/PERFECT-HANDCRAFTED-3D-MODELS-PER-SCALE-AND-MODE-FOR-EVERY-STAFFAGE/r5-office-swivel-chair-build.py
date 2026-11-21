"""🏗 Round 5, furniture/office-swivel-chair: the round-3 candidate builder fed with the round-5 body (`r5-office-swivel-chair-body.py`); writes the same candidate document and volumes source.

Usage: python3 r5-office-swivel-chair-build.py
@see ./r3-office-swivel-chair-build.py
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def load(path, name):
    spec = importlib.util.spec_from_file_location(name, os.path.join(HERE, path))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


BUILD = load("r3-office-swivel-chair-build.py", "chair_build_r3")
_r3_module = BUILD.module


def module(name):
    return load("r5-office-swivel-chair-body.py", "chair_body_r5") if name == "body" else _r3_module(name)


BUILD.module = module

if __name__ == "__main__":
    BUILD.main()
