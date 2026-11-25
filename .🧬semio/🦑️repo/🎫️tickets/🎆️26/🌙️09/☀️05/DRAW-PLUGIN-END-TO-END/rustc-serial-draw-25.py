#!/usr/bin/env python3
"""🧵 Run the pinned compiler with a serial frontend in the local Draw build."""
import os
import sys

command = sys.argv[1:]
if '--crate-name' in command:
    command += ['-Zthreads=1']
os.execv(command[0], command)
