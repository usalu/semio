from pathlib import Path
import av
from PIL import Image, ImageDraw

root = Path(__file__).resolve().parent / '🗑️generated/sections-preview/62efe597fff3'
indices = [3, 5, 7, 13, 15, 17]
for name in ('UniformSectionOverview', 'UniformRoomOpenings'):
    sheet = Image.new('RGB', (1600, 1410), '#04141a')
    draw = ImageDraw.Draw(sheet)
    with av.open(root / 'videos/900p10' / (name + '.mp4')) as container:
        for index, frame in enumerate(container.decode(video=0)):
            if index in indices:
                position = indices.index(index)
                x, y = (position % 2) * 800, (position // 2) * 470
                sheet.paste(frame.to_image().resize((800, 450)), (x, y + 20))
                draw.text((x + 8, y + 3), name + ' at ' + str(frame.time) + ' s', fill='white')
    sheet.save(root / (name + '-motion-contact.png'))
