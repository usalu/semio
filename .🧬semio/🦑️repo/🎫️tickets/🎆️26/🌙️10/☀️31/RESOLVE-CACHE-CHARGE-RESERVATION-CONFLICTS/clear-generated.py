import shutil
from pathlib import Path

folder = Path(__file__).resolve().parent / "🗑️generated"
shutil.rmtree(folder, ignore_errors=True)
print("gone", not folder.exists())
