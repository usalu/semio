const fs=require("fs");const p=process.argv[2]+"/⚛️locale-picker.tsx";let s=fs.readFileSync(p,"utf8");
function once(a,b){if(s.split(a).length!==2){console.error("count:",a);process.exit(1)}s=s.replace(a,()=>b)}
once(`          {DEMONSTRATOR_LOCALES.map((locale, index) => (
            <span key={locale} lang={locale}>
              {index === 0 ? "" : " / "}
              {demonstratorText("demonstrator.locale.choose", locale)}
            </span>
          ))}`,`          {DEMONSTRATOR_LOCALES.map((locale) => (
            <span key={locale} lang={locale} className="block">
              {demonstratorText("demonstrator.locale.choose", locale)}
            </span>
          ))}`);
once('<div className="flex gap-double">\n          {DEMONSTRATOR_LOCALES.map((locale) => (\n            <button key={locale} type="button" lang','<div role="group" aria-labelledby="demonstrator-locale-picker-title" className="flex gap-double">\n          {DEMONSTRATOR_LOCALES.map((locale) => (\n            <button key={locale} type="button" lang');
fs.writeFileSync(p,s);
