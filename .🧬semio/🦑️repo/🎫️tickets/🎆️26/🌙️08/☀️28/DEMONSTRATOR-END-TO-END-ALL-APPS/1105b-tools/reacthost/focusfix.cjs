const fs=require("fs");const p=process.argv[2]+"/🟦️.tsx";let s=fs.readFileSync(p,"utf8");
function once(a,b){if(s.split(a).length!==2){console.error("count:",a);process.exit(1)}s=s.replace(a,()=>b)}
once('import { mountUiRoot, useUiCallback as useCallback, useUiMemo as useMemo, useUiState as useState }','import { mountUiRoot, useUiCallback as useCallback, useUiEffect as useEffect, useUiMemo as useMemo, useUiState as useState }');
once(`  const [locale, setLocale] = useState<DemonstratorLocale | undefined>(INITIAL_LOCALE);
  const chooseLocale = useCallback((chosen: DemonstratorLocale) => {
    commitDemonstratorLocale(chosen);
    setLocale(chosen);
  }, []);
  return locale === undefined ? <DemonstratorLocalePicker onChoose={chooseLocale} /> : <DemonstratorLocaleLanding locale={locale} onChooseLocale={chooseLocale} />;`,
`  const [locale, setLocale] = useState<DemonstratorLocale | undefined>(INITIAL_LOCALE);
  const [picked, setPicked] = useState(false);
  const chooseLocale = useCallback((chosen: DemonstratorLocale) => {
    commitDemonstratorLocale(chosen);
    setLocale(chosen);
  }, []);
  const pickLocale = useCallback((chosen: DemonstratorLocale) => {
    setPicked(true);
    chooseLocale(chosen);
  }, [chooseLocale]);
  return locale === undefined ? <DemonstratorLocalePicker onChoose={pickLocale} /> : <DemonstratorLocaleLanding locale={locale} onChooseLocale={chooseLocale} focusHeading={picked} />;`);
once('function DemonstratorLocaleLanding({ locale, onChooseLocale }: { readonly locale: DemonstratorLocale; readonly onChooseLocale: (locale: DemonstratorLocale) => void }) {',
`function DemonstratorLocaleLanding({ locale, onChooseLocale, focusHeading }: { readonly locale: DemonstratorLocale; readonly onChooseLocale: (locale: DemonstratorLocale) => void; readonly focusHeading: boolean }) {
  useEffect(() => {
    if (!focusHeading) return undefined;
    const frame = requestAnimationFrame(() => document.querySelector<HTMLElement>('h1[data-slot="app-name"]')?.focus());
    return () => cancelAnimationFrame(frame);
  }, [focusHeading]);`);
once('<h1 data-slot="app-name" className="m-0 px-single','<h1 data-slot="app-name" tabIndex={-1} className="m-0 px-single focus:outline-none');
fs.writeFileSync(p,s);
