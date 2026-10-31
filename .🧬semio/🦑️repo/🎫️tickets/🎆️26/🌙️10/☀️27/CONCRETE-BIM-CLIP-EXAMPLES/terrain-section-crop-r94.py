"""🔎 Inspect retained source-document level arrows without modifying the source image."""
from pathlib import Path
from PIL import Image
p=Path(__file__).parent;im=Image.open(p/'🗑️generated/register-r9-section23.png');im.crop((130,1300,2290,1700)).save(p/'🗑️generated/terrain-section-ground-r94.png');print('[DEBUG] '+str(im.size))
