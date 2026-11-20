(() => {
  const params = new URLSearchParams(location.search);
  const token = Number(params.get("matrix"));
  if (!document.referrer || window === parent) return;
  const runner = new URL(document.referrer).origin;
  const send = (type, data = {}) => parent.postMessage({type, token, ...data}, runner);
  const pause = ms => new Promise(resolve => setTimeout(resolve, ms));
  const find = selector => document.querySelector(selector);
  const pageErrors = [];
  addEventListener("keydown", event => { if (event.isTrusted) send("matrix-key"); }, true);
  addEventListener("error", event => { if (event.message) pageErrors.push(event.message); });
  addEventListener("unhandledrejection", event => pageErrors.push(String(event.reason)));
  const load = src => new Promise((resolve,reject)=>{
    const script = document.createElement("script");
    script.src = src;
    script.onload = resolve;
    script.onerror = () => reject(new Error("Could not load " + src));
    document.head.append(script);
  });
  const paint = () => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));
  async function settled(root, delay) {
    if (delay) await pause(delay);
    const start = performance.now();
    do {
      await paint();
      if (root.firstChild && !document.querySelector("[data-shared-border-pending]")) return;
    } while (performance.now() - start < 1000);
  }
  async function waitFor(check, description) {
    const deadline = performance.now() + 75000;
    do {
      const result = check();
      if (result) return result;
      const viteError = find("vite-error-overlay")?.shadowRoot?.textContent;
      if (viteError) throw new Error(viteError.trim().slice(0, 500));
      await pause(125);
    } while (performance.now() < deadline);
    throw new Error("Timed out waiting for " + description);
  }
  function readLocale(app) {
    if (app === "hub") return find("#admin-locale-switch")?.textContent.trim().toLowerCase() ?? "";
    const declared = document.documentElement.lang.toLowerCase().split("-")[0];
    if (declared) return declared;
    const languages = new Set([...document.querySelectorAll("[lang]")].map(el => el.lang.toLowerCase().split("-")[0]));
    return languages.size === 1 ? [...languages][0] : "";
  }
  const selector = 'button, [role="radio"], [role="option"], label';
  const choose = locale => {
    const text = locale === "de" ? /^(deutsch|german|de)$/i : /^(english|englisch|en)$/i;
    return [...document.querySelectorAll(selector)].find(el => text.test(el.textContent.trim()) && el.getBoundingClientRect().width);
  };
  const ready = {
    cad: () => find("#root aside #cmd"),
    course: () => find("#root button, #root input, #root [role=button]"),
    demo: () => find('[data-slot="app-name"]'),
    elements: () => window.round3,
    hub: () => find("#admin-tab-overview, #admin-tab-switch"),
    os: () => document.body?.dataset.ready,
    play: () => find('[data-slot="play-app-count"]'),
    staffage: () => find(".staffage-app[lang]"),
  };
  let busy = false;
  addEventListener("message", async event => {
    if (event.source !== parent || event.origin !== runner || event.data?.type !== "matrix-run" || event.data.token !== token || busy) return;
    const {app, locale, width, height, attempt} = event.data;
    busy = true;
    const fail = async message => {
      const result={app,locale,width,height,attempt,at:new Date().toISOString(),url:location.href,errors:[message,...pageErrors]};
      await fetch("/__matrix_result",{method:"POST",headers:{"content-type":"application/json"},body:JSON.stringify(result)});
      send("matrix-error", {message});
    };
    const meta = {app,locale,width,height};
    let lastHeartbeat = -Infinity, actual = "", originalCheck;
    const progress = (message, force = false) => {
      const now = performance.now();
      if (!force && now - lastHeartbeat < 1500) return;
      lastHeartbeat = now;
      send("matrix-progress", {message});
      void fetch("/__matrix_progress",{method:"POST",headers:{"content-type":"application/json"},body:JSON.stringify({...meta,message}),keepalive:true}).catch(()=>{});
    };
    const json = async (path, request) => {
      const response = await fetch(path,request);
      if (!response.ok) throw new Error(path + ": HTTP " + response.status);
      return response.json();
    };
    const post = (path, value) => json(path,{method:"POST",headers:{"content-type":"application/json"},body:JSON.stringify(value)});
    const storyApp = app === "elements" || app === "os";
    const checkpoint = storyApp ? "/__matrix_checkpoint?" + new URLSearchParams({locale,width,height}) : undefined;
    try {
      progress("Waiting for app content", true);
      if (!window.silhouette) await load("/__matrix_probe.js");
      if (app === "os" && !window.osRun) {
        await load("/__matrix_os_sweep.js");
        await waitFor(()=>window.osRun,"OS story harness");
      }
      if (locale !== "unlocalized") {
        const localeState = () => (ready[app]() && readLocale(app)) || choose(locale);
        await waitFor(localeState, app + " language controls");
        if (readLocale(app) !== locale) {
          if (app === "hub" && find("#admin-locale-switch")) {
            find("#admin-locale-switch").click();
          } else if (!choose(locale)) {
            const trigger = [...document.querySelectorAll("button,[role=combobox]")].find(el => /language|sprache|locale/i.test((el.getAttribute("aria-label") || "") + " " + (el.textContent || "")));
            trigger?.click();
          }
          (await waitFor(() => choose(locale), app + " " + locale + " language choice")).click();
        }
        await waitFor(() => readLocale(app) === locale, app + " locale " + locale);
      }
      await waitFor(ready[app], app + " content");
      if (app === "staffage") progress("Waiting for initial Staffage paint", true);
      await paint();
      actual = readLocale(app);
      const stories = [], report = [], errors = [];
      let targets = 0, focusChecks = 0, currentFocus = 0;
      originalCheck = silhouette.check;
      silhouette.check = (el, options) => {
        if (el.matches?.(":focus-visible")) currentFocus++;
        const checked = originalCheck(el, options);
        if (app === "hub" && el.id === "admin-tab-overview" && checked.issues.length) {
          const box = el.getBoundingClientRect();
          const detail = silhouette.segments(true).filter(({from,to}) =>
            Math.min(from.x,to.x) <= box.right+4 && Math.max(from.x,to.x) >= box.left-4 &&
            Math.min(from.y,to.y) <= box.bottom+4 && Math.max(from.y,to.y) >= box.top-4
          ).slice(0,20).map(({from,to,state})=>[state,from.x,from.y,to.x,to.y]);
          const ownPaths = [...el.querySelectorAll("svg[data-shared-border-overlay] path")].map(path=>[path.dataset.sharedBorderState,path.getAttribute("d"),getComputedStyle(path).stroke]);
          const ancestors = [];
          for (let node = el.parentElement; node && ancestors.length < 4; node = node.parentElement) {
            const rect = node.getBoundingClientRect(), style = getComputedStyle(node);
            ancestors.push([node.dataset.slot,rect.left,rect.top,rect.right,rect.bottom,style.overflowX,style.overflowY,node.scrollLeft]);
          }
          checked.diagnostic = {box:[box.left,box.top,box.right,box.bottom],segments:detail,ownPaths,ancestors,active:document.activeElement?.id,tabState:el.dataset.state};
        }
        return checked;
      };
      const combine = ({name,targets:count,focusChecks:focus,report:issues,error}) => {
        if (error) { errors.push(name + ": " + error); return; }
        targets += count;
        focusChecks += focus;
        report.push(...issues.map(issue=>({story:name,...issue})));
        stories.push({name,targets:count,failing:issues.length});
      };
      const sweep = async (name, root, limit) => {
        currentFocus = 0;
        const result = await silhouette.sweep(root,{limit,onResult:index=>progress(name + ": target " + (index+1))});
        return {name,targets:result.targets,focusChecks:currentFocus,report:result.report};
      };
      if (storyApp) {
        const previous = await json(checkpoint);
        const resumed = Object.keys(previous).length;
        if (resumed) progress("Resuming " + resumed + " completed stories", true);
        const visit = async (name, mount, limit) => {
          if (Object.hasOwn(previous,name)) { combine(previous[name]); return; }
          progress("Loading " + name);
          let story;
          try {
            const root = mount();
            await settled(root, app === "os" && name === "context-sub" ? 550 : 0);
            const renderError = root.querySelector("[data-render-error]");
            if (renderError) throw new Error(renderError.textContent);
            story = await sweep(name, root, limit);
          } catch(error) { story = {name,error:String(error)}; }
          await post(checkpoint,{key:name,story});
          combine(story);
        };
        if (app === "os") await osRun({visit});
        else {
          if (!window.runSweep) await load("/__matrix_elements_sweep.js");
          await runSweep("matrix", {visit});
        }
      } else if (app === "play") {
        const introduction = find('[data-slot="introduction-info-box"]');
        if (introduction) {
          combine(await sweep("onboarding", introduction, 10000));
          const skip = introduction.querySelector('[data-slot="introduction-close"]');
          if (!skip) throw new Error("Play onboarding Skip action was missing");
          skip.click();
          await waitFor(()=>!introduction.isConnected,"Play onboarding to close");
          await paint();
        }
        combine(await sweep("landing", document, 10000));
      } else if (app === "staffage") {
        sessionStorage.removeItem("staffage:" + innerWidth);
        window.__pumped = true;
        progress("Loading Staffage probe", true);
        if (!window.appSweep) await load("/__matrix_app_sweep.js");
        progress("Starting Staffage sweep", true);
        await appSweep("staffage", 50, index=>progress("Staffage: target " + (index+1), index === 0));
        const state = appState("staffage");
        combine({name:"staffage",targets:state.t,focusChecks:currentFocus,report:state.issues.map(issue=>({issue}))});
      } else combine(await sweep("current page", document, 10000));
      if (innerWidth !== width || innerHeight !== height) errors.push("Viewport mismatch: observed " + innerWidth + " × " + innerHeight);
      if (!targets) errors.push("No targets were checked; the app may not have loaded.");
      if (targets && !focusChecks) errors.push("No focus-visible checks ran; keyboard modality not established.");
      const result={app,locale,observedLocale:actual,width,height,attempt,actualWidth:innerWidth,actualHeight:innerHeight,url:location.href,at:new Date().toISOString(),targets,focusChecks,failures:report.length,stories,report,errors:[...pageErrors,...errors]};
      const saved=await fetch("/__matrix_result",{method:"POST",headers:{"content-type":"application/json"},body:JSON.stringify(result)});if(!saved.ok)throw new Error("Result persistence failed: "+saved.status);
      if (checkpoint) await json(checkpoint,{method:"DELETE"});
      progress("Completed " + targets + " targets", true);
      send("matrix-done",{result});
    } catch(error) { progress(String(error), true); await fail(String(error)); } finally {
      if (originalCheck) silhouette.check = originalCheck;
      busy = false;
    }
  });
  send("matrix-ready");
})();
