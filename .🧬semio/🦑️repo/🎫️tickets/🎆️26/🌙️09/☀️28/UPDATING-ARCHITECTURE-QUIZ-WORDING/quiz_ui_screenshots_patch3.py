import io, sys
p = sys.argv[1]
s = io.open(p, encoding="utf-8", newline="").read()
old = """      wrapped: [...document.querySelectorAll("table th[scope=col], table .quiz-nowrap")].flatMap((cell) => {
        const range = document.createRange();
        range.selectNodeContents(cell);
        const lines = new Set([...range.getClientRects()].filter((rect) => rect.width > 0).map((rect) => Math.round(rect.top)));
        return lines.size > 1 ? [`${cell.textContent?.trim()} (${lines.size} lines)`] : [];
      }),"""
new = """      wrapped: [...document.querySelectorAll("table th[scope=col], table .quiz-nowrap")].flatMap((cell) => {
        const walker = document.createTreeWalker(cell, NodeFilter.SHOW_TEXT);
        const rects: DOMRect[] = [];
        for (let node = walker.nextNode(); node !== null; node = walker.nextNode()) {
          const range = document.createRange();
          range.selectNodeContents(node);
          rects.push(...[...range.getClientRects()].filter((rect) => rect.width > 0));
        }
        const style = getComputedStyle(cell);
        const line = Number.parseFloat(style.lineHeight) || Number.parseFloat(style.fontSize) * 1.2;
        const lines = rects.length === 0 ? 0 : Math.round((Math.max(...rects.map((rect) => rect.bottom)) - Math.min(...rects.map((rect) => rect.top))) / line);
        return lines > 1 ? [`${cell.textContent?.trim()} (${lines} lines)`] : [];
      }),"""
if s.count(old) != 1:
    sys.exit("anchor")
s = s.replace(old, new)
io.open(p, "w", encoding="utf-8", newline="").write(s)
print("patched")
