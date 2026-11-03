set -u
export PATH="$HOME/.local/share/semio/launcher/bin:$PATH"
cd /mnt/c/git/semio || exit 1
git status --short -- .cache 2>/dev/null | head -3
export SEMIO_LANGUAGE=en; timeout 280 repo --help 2>&1 | head -25
echo "exit=$?"
