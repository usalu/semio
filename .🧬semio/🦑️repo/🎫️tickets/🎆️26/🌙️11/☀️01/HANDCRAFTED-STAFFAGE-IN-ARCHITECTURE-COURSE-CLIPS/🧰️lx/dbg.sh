set -u
R=/mnt/c/git/semio
T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
export SEMIO_REPO_BINARY="$T/🧰️lx/repo-shim.sh" PYTHONDONTWRITEBYTECODE=1
cd "$R"
python3 -I "$T/🧰️lx/cancellation.py" "$R" 2>&1 | grep -B3 -A12 "ERROR: test_progress" | head -40
