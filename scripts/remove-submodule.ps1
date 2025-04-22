$submodulePath = "examples/geometry"
git rm $submodulePath
git config --remove-section submodule.$submodulePath 2>$null
git config -f .gitmodules --remove-section submodule.$submodulePath 2>$null
Remove-Item -Path ".git\modules\$submodulePath" -Recurse -Force -ErrorAction SilentlyContinue