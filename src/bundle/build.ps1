.venv/Scripts/activate.ps1
pyinstaller --clean -y -n "semio" -c -i semio.ico --add-binary="restproxy.exe;." semio.py