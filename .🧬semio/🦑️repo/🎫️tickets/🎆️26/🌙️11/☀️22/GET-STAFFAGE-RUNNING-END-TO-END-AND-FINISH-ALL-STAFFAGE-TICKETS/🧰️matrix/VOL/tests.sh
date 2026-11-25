#!/bin/zsh
# 🧪️ The suites of `proof.sh` for the 27 in their own grant of 2500 MB, asked for again when the queue gives up: tests.sh
source "${0:A:h}/env.sh"; cd "$R"
until [[ -e "$GEN/tests.started" ]]; do SEMIO_LANGUAGE=en repo capacity run --memory="${MB:-2500}" -- timeout 30000 zsh -c 'touch "$GEN/tests.started"; STEPS=tests "$W/proof.sh" --inside'; sleep 5; done
