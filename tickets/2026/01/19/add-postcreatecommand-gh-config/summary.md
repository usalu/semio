# Summary

Added ownership fix for the mounted gh config directory to [post-create.sh](.devcontainer/post-create.sh).

## Change
Added to `.devcontainer/post-create.sh`:
```bash
echo "Fixing ownership of mounted config directories..."
sudo chown -R vscode:vscode /home/vscode/.config/gh || true
```

This fixes permission issues with the mounted `/home/vscode/.config/gh` volume which may have root ownership when first created.
