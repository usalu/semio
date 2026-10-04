"""Rewrites the end-to-end spec `🥞️layered-home` from the grid-map camera to the play behaviour (pan between the cards,
glide to the page of a hovered card). One-off edit of the overview-like-play revision; retries the write because another
process may hold the file for a moment."""
import io
import pathlib
import sys
import time

root = pathlib.Path(__file__).resolve().parents[7]
path = root / "🎓️teaching" / "🏛️architecture" / "❓️quiz" / "🧪️tests" / "🥞️layered-home" / "🟦️.ts"
text = path.read_bytes().decode("utf-8")
nl = "\r\n" if "\r\n" in text else "\n"


def swap(old: str, new: str) -> None:
    global text
    old, new = old.replace("\n", nl), new.replace("\n", nl)
    assert text.count(old) == 1, (old[:60], text.count(old))
    text = text.replace(old, new)


swap(
    """ * page of every card, mounted and live but inert at rest — a map the mouse moves a camera over without ever losing a
 * page, still for a device that asks for reduced motion and for a learner who switched it off. Hovering or
 * keyboard-focusing a card shows its page clear and never starts a run; a page opens full size by its hash or its card
""",
    """ * page of every card, each as large as the screen, mounted and live but inert at rest. Between the cards the mouse pans
 * those pages under the glass, as on semio-tech play; hovering or keyboard-focusing a card brings its page to the
 * screen, clear, and never starts a run; a device that asks for reduced motion gets neither pan nor glide. A page opens
 * full size by its hash or its card
""",
)
start = text.index("/** 🔭️ How far the camera magnifies the desktop map about the pointer")
end = text.index("async function overview(device: Device)")
text = (
    text[:start]
    + """/** ⏳️ How long a pan needs to leave its place visibly: asserting stillness earlier would pass for a pan that had only
 * not started yet. */
const PAN_SETTLE_MS = 800;

/** 📍️ Where the page of `id` lies relative to the overview, in pixels: its left and top edge and its size. */
async function placeOf(device: Device, id: string): Promise<{ readonly x: number; readonly y: number; readonly width: number; readonly height: number }> {
  const [frame, box] = [await boxOf(device.page.locator("[data-layered-overview]")), await boxOf(pane(device.page, id))];
  return { x: box.x - frame.x, y: box.y - frame.y, width: box.width, height: box.height };
}

""".replace("\n", nl)
    + text[end:]
)
swap(
    """  await expect(root).toHaveAttribute("data-rest", "grid");
  expect(await root.locator("[data-layered-card]")""",
    """  await expect(root).toHaveAttribute("data-pan", "pointer");
  expect(await root.locator("[data-layered-card]")""",
)
start = text.index('test("the mouse moves a camera over the map of the pages and never loses one')
end = text.index('test("hovering or focusing a card shows its page clear and never starts a run"')
text = (
    text[:start]
    + """test("between the cards the mouse pans the screen-sized pages under the glass; on a card its page comes to the screen, clear; reduced motion keeps them still", async ({ device }) => {
  const learner = await device("en");
  await enter(learner, { kind: "pseudonym", handle: handle("Home Pan") }, true);
  const root = await overview(learner);
  await expect(root).toHaveAttribute("data-pan", "pointer");
  const veil = root.locator("[data-layered-veil]");
  const frame = await boxOf(root);
  const cards = await Promise.all(PAGES.map((id) => boxOf(card(learner.page, id).locator("[data-overview-card]"))));
  for (const [index, id] of PAGES.entries()) {
    const place = await placeOf(learner, id);
    expect([Math.round(place.width), Math.round(place.height)], `${id} is as large as the overview`).toEqual([Math.round(frame.width), Math.round(frame.height)]);
    expect(Math.max(Math.abs(place.x - (index % 3) * frame.width), Math.abs(place.y - Math.floor(index / 3) * frame.height)), `${id} lies in its cell of the strip, the first page on the screen`).toBeLessThan(1.5);
  }

  const offPan = async (id: string, at: { readonly x: number; readonly y: number }): Promise<number> => {
    const index = PAGES.indexOf(id);
    const [place, across, down] = [await placeOf(learner, id), ((at.x - frame.x) / frame.width) * 2, ((at.y - frame.y) / frame.height) * 2];
    return Math.max(Math.abs(place.x - ((index % 3) - across) * frame.width), Math.abs(place.y - (Math.floor(index / 3) - down) * frame.height));
  };
  for (const [fx, fy, shown] of [[1, 1, "prefs"], [1, 0, "intro"], [0.5, 1, fourth!], [0, 0, "learner"]] as const) {
    const at = { x: frame.x + Math.min(frame.width - 2, Math.max(2, fx * frame.width)), y: frame.y + Math.min(frame.height - 2, Math.max(2, fy * frame.height)) };
    await learner.page.mouse.move(at.x, at.y, { steps: 4 });
    await expect.poll(() => offPan(shown, at), { message: `the pointer at ${fx}, ${fy} pans to ${shown}` }).toBeLessThan(1.5);
    const place = await placeOf(learner, shown);
    expect(Math.max(Math.abs(place.x), Math.abs(place.y)), `${shown} fills the screen behind the glass`).toBeLessThan(6);
    await expect(veil).toHaveAttribute("data-veil", "whole");
    await expect(veil).toBeVisible();
    expect(await Promise.all(PAGES.map((id) => boxOf(card(learner.page, id).locator("[data-overview-card]")))), "the cards stay where they are").toEqual(cards);
  }

  const onScreen = async (id: string): Promise<number> => {
    const place = await placeOf(learner, id);
    return Math.max(Math.abs(place.x), Math.abs(place.y));
  };
  const board = card(learner.page, "board").locator("[data-overview-card]");
  await board.hover();
  await expect(card(learner.page, "board")).toHaveAttribute("data-revealed", "");
  await expect.poll(() => onScreen("board"), { message: "the page of the hovered card glides to the screen" }).toBeLessThan(0.5);
  await expect(veil).toHaveAttribute("data-veil", "clear");
  await expect(veil).toBeHidden();
  const held = await boxOf(board);
  await learner.page.mouse.move(held.x + 8, held.y + held.height - 8, { steps: 3 });
  await learner.page.waitForTimeout(PAN_SETTLE_MS);
  expect(await onScreen("board"), "the page stays while the mouse moves on its card").toBeLessThan(0.5);
  const corner = { x: frame.x + frame.width - 2, y: frame.y + 2 };
  await learner.page.mouse.move(corner.x, corner.y, { steps: 4 });
  await expect(veil).toHaveAttribute("data-veil", "whole");
  await expect.poll(() => offPan("intro", corner), { message: "between the cards the pages pan again" }).toBeLessThan(1.5);

  await learner.page.emulateMedia({ reducedMotion: "reduce" });
  await expect(root).toHaveAttribute("data-pan", "none");
  await learner.page.mouse.move(frame.x + 2, frame.y + frame.height - 2, { steps: 4 });
  await learner.page.waitForTimeout(PAN_SETTLE_MS);
  expect(await offPan("intro", corner), "a device that asks for reduced motion gets no pan").toBeLessThan(1.5);
  await board.hover();
  await expect(card(learner.page, "board")).toHaveAttribute("data-revealed", "");
  expect(await onScreen("board"), "and no glide: the page is there at once").toBeLessThan(0.5);
  await expect(veil).toHaveAttribute("data-veil", "clear");
  await learner.page.emulateMedia({ reducedMotion: "no-preference" });
  await expect(root).toHaveAttribute("data-pan", "pointer");
  const last = { x: frame.x + frame.width - 2, y: frame.y + frame.height - 2 };
  await learner.page.mouse.move(last.x, last.y, { steps: 4 });
  await expect.poll(() => offPan("prefs", last), { message: "the pages pan again" }).toBeLessThan(1.5);
});

""".replace("\n", nl)
    + text[end:]
)
swap(
    """    await expect.poll(async () => Math.round((await boxOf(pane(learner.page, id))).width)).toBe(Math.round(frame.width));
""",
    """    await expect.poll(async () => Object.values(await boxOf(pane(learner.page, id))).map(Math.round), { message: `the page of ${id} fills the overview` }).toEqual(Object.values(frame).map(Math.round));
""",
)
data = text.encode("utf-8")
for attempt in range(20):
    try:
        with io.open(path, "r+b") as handle:
            handle.seek(0)
            handle.write(data)
            handle.truncate()
        break
    except OSError as error:
        last = error
        time.sleep(0.5)
else:
    raise last
assert path.read_bytes() == data
sys.stdout.write(f"written {len(data)} bytes; grid leftovers: {text.count('data-rest')} {text.count('shownCells')} {text.count('CAMERA')} {text.count('BACKGROUND_SWITCH')}\n")
