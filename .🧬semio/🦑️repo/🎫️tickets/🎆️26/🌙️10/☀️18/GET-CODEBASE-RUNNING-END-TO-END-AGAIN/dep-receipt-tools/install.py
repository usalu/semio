import os, shutil, sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from stage import ROOT, STAGE, FILES, GEN
from stage_tests import TEST_FILES, TESTS

MAP = {
    "schema.json": FILES["schema.json"],
    "generation.ts": FILES["generation.ts"],
    "storage.ts": FILES["storage.ts"],
    "incremental.ts": FILES["incremental.ts"],
    "depstorage.ts": FILES["depstorage.ts"],
    "bootstrap.ts": FILES["bootstrap.ts"],
    "tests.ts": TEST_FILES["tests.ts"],
    "fixtures.json": TEST_FILES["fixtures.json"],
    "ingress.ts": TEST_FILES["ingress.ts"],
    "followup.ts": TEST_FILES["followup.ts"],
    "oracle.py": TESTS + "/🐍️keying.py",
    "incremental-tests.ts": TEST_FILES["incremental-tests.ts"],
}
ORDER = ["schema.json", "generation.ts", "storage.ts", "incremental.ts", "depstorage.ts", "bootstrap.ts", "fixtures.json", "tests.ts", "ingress.ts", "followup.ts", "incremental-tests.ts", "oracle.py"]


def install(target):
    pending = []
    for name in ORDER:
        destination = target + MAP[name][len(ROOT):]
        os.makedirs(os.path.dirname(destination), exist_ok=True)
        temporary = destination + ".dr-tmp"
        shutil.copyfile(STAGE + "/" + name, temporary)
        pending.append((temporary, destination))
    for temporary, destination in pending:
        os.replace(temporary, destination)
    print("installed", len(pending), "files into", target)


install(sys.argv[1].rstrip("/"))
