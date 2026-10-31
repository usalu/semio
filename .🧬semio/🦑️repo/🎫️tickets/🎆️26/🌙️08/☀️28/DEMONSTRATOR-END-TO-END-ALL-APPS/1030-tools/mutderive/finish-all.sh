#!/bin/bash
D="$(dirname "$0")"
"$D/check-retained.sh"; "$D/check-transients.sh"; "$D/check-gen3d.sh"; "$D/home-fold-test.sh"; "$D/law-epw-home.sh"; "$D/check-hub.sh"
