#!/bin/bash
T="$(cd "$(dirname "$0")" && pwd)"
for b in fw_fix3 root_aa root_ab fw_aa fw_ab; do "$T/run.sh" nat $b; done
