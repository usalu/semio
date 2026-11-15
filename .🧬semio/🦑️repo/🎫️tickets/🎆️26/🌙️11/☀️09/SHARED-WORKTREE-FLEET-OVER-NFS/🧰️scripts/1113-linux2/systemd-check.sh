#!/bin/bash
# runs in the bun container: generates the units from the module copy at /mir
mkdir -p /out && cd /mir && bun /s/gen-units.ts && ls /out && cat /out/replica.service
