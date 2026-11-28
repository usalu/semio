() => {
  window.__ctx = [];
  window.__dl = [];
  window.__fileReq = [];
  const blobs = new Map();
  const create = URL.createObjectURL.bind(URL);
  URL.createObjectURL = (obj) => { const u = create(obj); blobs.set(u, obj); return u; };
  window.__blobs = blobs;
  HTMLAnchorElement.prototype.click = function () {
    if (this.download) {
      const blob = blobs.get(this.href);
      const entry = { name: this.download, size: blob ? blob.size : -1, type: blob ? blob.type : "", href: this.href.slice(0, 40), at: Date.now() };
      window.__dl.push(entry);
      if (blob) blob.arrayBuffer().then((b) => { entry.bytes = new Uint8Array(b); entry.head = new TextDecoder().decode(entry.bytes.slice(0, 80)); });
    }
  };
  const nativeClick = HTMLInputElement.prototype.click;
  HTMLInputElement.prototype.click = function () {
    if (this.type === "file") {
      this.id = "__file-input"; this.style.display = "none";
      window.__fileReq.push({ accept: this.accept, multiple: this.multiple, at: Date.now() });
      document.body.appendChild(this);
      return;
    }
    return nativeClick.call(this);
  };
  window.__keys = [];
  window.addEventListener('keydown', (e) => window.__keys.push([e.key, e.code, e.metaKey, e.shiftKey, (e.target && e.target.tagName) || '', e.defaultPrevented, Date.now() % 100000]), true);
  window.__h = {
    wait: (ms) => new Promise((r) => setTimeout(r, ms)),
    text: () => document.body.innerText,
    find(label, scope) {
      const root = scope || document;
      const norm = (s) => String(s || "").trim();
      return [...root.querySelectorAll("button,[role=tab],[role=menuitem],[role=option],[role=button]")].find((b) => norm(b.getAttribute("aria-label")) === label || norm(b.innerText) === label || norm(b.innerText).split("\n")[0].trim() === label);
    },
    click(label, scope) {
      const b = this.find(label, scope);
      if (!b) throw new Error("no button: " + label + " in " + document.body.innerText.slice(0, 300).replace(/\n/g, "|"));
      b.click();
      return label;
    },
    steps() {
      const m = document.body.innerText.match(/(\d+)\/(\d+) Steps · ([\d.]+) m³/);
      return m ? { k: +m[1], n: +m[2], vol: +m[3] } : null;
    },
    async until(fn, ms = 20000, what = "condition") {
      const t0 = Date.now(), end = t0 + ms;
      while (Date.now() < end) { let v; try { v = fn(); } catch { v = null; } if (v) { this.ms = Date.now() - t0; if (typeof v === "object") v.ms = this.ms; return v; } await this.wait(250); }
      throw new Error("timeout waiting for " + what + " :: " + document.body.innerText.slice(0, 600).replace(/\n/g, "|"));
    },
  };

  Object.assign(window.__h, {
    ok(cond, msg) { if (!cond) throw new Error("ASSERT " + msg); return msg; },
    json: (v) => JSON.stringify(v),
    async actions() { if (!this.find("Add Step…")) { this.click("Actions"); await this.until(() => this.find("Add Step…"), 20000, "actions panel"); } },
    async form(label) { await this.actions(); this.click(label); await this.wait(700); },
    async pick(option) {
      const boxes = [...document.querySelectorAll("[role=combobox]")].filter((b) => b.getAttribute("aria-label") !== "Example");
      const box = boxes[boxes.length - 1];
      if (!box) throw new Error("ASSERT no form combobox");
      box.click();
      const opt = await this.until(() => this.find(option), 8000, "option " + option);
      opt.click();
      await this.wait(500);
    },
    async exec() { this.click("Execute"); await this.wait(800); },
    setInput(el, value) {
      const setter = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, "value").set;
      el.focus(); setter.call(el, String(value));
      el.dispatchEvent(new Event("input", { bubbles: true })); el.dispatchEvent(new Event("change", { bubbles: true }));
      el.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", bubbles: true })); el.blur();
    },
    labels: () => [...document.querySelectorAll("[data-slot=tree-label]")].map((e) => e.innerText.trim()),
    async stepsNow(pred, ms = 30000, what = "steps") { return this.until(() => { const s = this.steps(); return s && pred(s) ? s : null; }, ms, what); },
    row(label) { return this.find(label).closest("[data-slot=tree-row-content]"); },
    notices: () => [...document.querySelectorAll("[role=status],[role=alert],[data-slot=toast],[data-sonner-toast],[data-slot=notice]")].map((e) => e.innerText.trim()).filter(Boolean),
    async ctxmenu(el, item) {
      const r = el.getBoundingClientRect();
      el.dispatchEvent(new MouseEvent("contextmenu", { bubbles: true, cancelable: true, clientX: r.x + 10, clientY: r.y + 5, button: 2 }));
      const mi = await this.until(() => [...document.querySelectorAll("[role=menuitem]")].find((e) => e.innerText.trim().includes(item)), 8000, "menu " + item);
      mi.click(); await this.wait(600);
    },
    ready: () => document.querySelectorAll("canvas").length >= 1 && !/Loading document/.test(document.body.innerText) && !!document.querySelector("[role=combobox]"),
  });
}
