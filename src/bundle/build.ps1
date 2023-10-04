.venv/Scripts/activate.ps1
copy-item "pyinstallerhooks\*" ".venv\Lib\site-packages\PyInstaller\hooks"
pyinstaller --clean -y -n "semio" -c -i semio.ico --add-binary="restproxy.exe;." cli.py