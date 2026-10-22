#!/usr/bin/env python3
"""Copy of the repo-doctor set whose dependency probe takes the input file names from the package manager seam (pm-seam neutrality law: no production file outside the adapters names a lock or configuration file)."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/repo-doctor-tools", "repo-doctor")
P = T + "/staged/probe-dependencies.ts"
patch(P, 'import type {DoctorProbeV1} from "../\U0001F7E6️.ts";\n', 'import type {DoctorProbeV1} from "../\U0001F7E6️.ts";\nimport {selectPackageManagerV1} from "../../../../⚡️caching/\U0001F680️bootstrap/\U0001F4E6️dependencies/\U0001F50C️managers/\U0001F4DA️catalog/\U0001F7E6️.ts";\n')
patch(P, 'const INPUTS=["package.json","bun.lock","bunfig.toml"],READY=', 'const manager=selectPackageManagerV1("javascript"),INPUTS=[...manager.manifestFileNames(),manager.primaryLockName(),...manager.configurationNames("scope")],READY=')
print("repo-doctor copy:", T)
