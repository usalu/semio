#!/bin/zsh
# 🕸️ Writes the repair ops of `used-climbing-net-pyramid` from the document of its generator before this repair.
source "${0:A:h}/../env.sh"
export ASSET_FILE=$("$W/pristine.sh" used-climbing-net-pyramid "$U/../USED-PARK-OBJECTS-STAFFAGE-COLLECTION/assets/used-climbing-net-pyramid/author-net-pyramid.py" | tail -1)
python3 "$W/make/used-climbing-net-pyramid.py" || exit 1
