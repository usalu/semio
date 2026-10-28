"""🔍 Audit native table chrome, numbering, and page-body clearance."""
import sys
import pdfplumber
from pypdf import PdfReader


def verify(path):
    reader = PdfReader(path)
    assert len(reader.pages) == 3
    lang = reader.trailer['/Root']['/Lang']
    titles = ['Beispiel', 'Tabelle A', 'Tabelle B'] if lang == 'de-DE' else ['Example', 'Table A', 'Table B']
    expected = [(0, titles[0], '1.1.a'), (1, titles[1], '1.2.a'), (1, titles[2], '1.2.b')]
    with pdfplumber.open(path) as pdf:
        for index, title, number in expected:
            page = pdf.pages[index]
            words = page.extract_words()
            match = [word for word in words if word['text'] == number]
            assert len(match) == 1, (title, number, [(w['text'], w['top']) for w in words if '1.' in w['text']])
            top = match[0]['top']
            line = ' '.join(word['text'] for word in words if abs(word['top']-top)<5)
            assert title in line, (title, number, line)
            print('PASS native chrome', index+1, line)
        for index, page in enumerate(pdf.pages):
            body = [char for char in page.chars if char['top'] > 70 and char['bottom'] < 790]
            assert body
            print('PAGE', index+1, 'body bottom mm', round(max(char['bottom'] for char in body)/(72/25.4), 2))
    print('PASS three-page native table titles, framework numbering and readable body')


for argument in sys.argv[1:]:
    verify(argument)
