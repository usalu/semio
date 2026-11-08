// #region 🧲️Header
/** 🏛️ Entwerfen mit Bestand landing partner/funding credits — brand-exclusive footer items, not part of the shared `@semio-tech/ui-react` component library. */
// #endregion 🧲️Header

import type { NavbarItem } from "@semio-tech/ui-react";
import { demonstratorFooterItems } from "./🪧️brand.ts";
import { demonstratorFormat, demonstratorText, type DemonstratorLocale } from "./🔨️modules/🌐️locale/🟦️.ts";

type PartnerCredit = ReturnType<typeof demonstratorFooterItems>[number];
type PartnerLogo = PartnerCredit["logos"][number];

/** 🏛️ The credit `id` of the shared partner table, worded and linked in `locale`; an unknown id is a defect and throws. */
function partnerCredit(id: PartnerCredit["id"], locale: DemonstratorLocale): PartnerCredit {
  const credit = demonstratorFooterItems(locale).find((item) => item.id === id);
  if (credit === undefined) throw new Error(`demonstrator partner credit ${id} is not in the shared table`);
  return credit;
}

/** 🏛️ One partner logo, light and dark variant, linking out to the partner's page in a new tab. */
function PartnerLogoLink({ logo, locale }: { readonly logo: PartnerLogo; readonly locale: DemonstratorLocale }) {
  return (
    <a
      href={logo.href}
      target="_blank"
      rel="noopener noreferrer"
      className="inline-flex h-tiny items-center rounded-sm hover:opacity-80 transition-opacity"
      aria-label={demonstratorFormat("demonstrator.footer.newTab", locale, { name: logo.alt })}
    >
      <span className="inline-flex shrink-0 items-center h-full">
        <img src={logo.src} alt="" className="h-full w-auto dark:hidden" />
        {logo.darkSrc !== undefined && <img src={logo.darkSrc} alt="" className="hidden h-full w-auto dark:block" />}
      </span>
    </a>
  );
}

//#region 🏛️FundedByZukunftBauFooterItem
/**
 * 🏛️ A plain (non-margined) footer item — the caller sits it between the bottom-middle command palette and
 * the bottom-right corner toggle by bracketing it with two `navbarFillItem` spacers, so it never claims the
 * exact corner pixel a floating corner `Panel` also anchors to. `relative z-40` matches `panel-tabs`' z-index so it
 * never disappears behind an anchored panel's own chrome. Rendered as a real `<a>` so it is announced as a link and
 * opens the funded project page in a new tab with middle or Ctrl click too.
 * 📱️ With `iconOnly` (the landing sets it on a phone) the credit shrinks to just its logo — the caption has no room.
 **/
export function fundedByZukunftBauFooterItem(key: string, locale: DemonstratorLocale, iconOnly: boolean): NavbarItem {
  const logo = partnerCredit("fundedByZukunftBau", locale).logos[0]!;
  return {
    key,
    className: "relative z-40",
    content: (
      <a
        href={logo.href}
        target="_blank"
        rel="noopener noreferrer"
        aria-label={demonstratorFormat("demonstrator.footer.newTab", locale, { name: `${demonstratorText("demonstrator.footer.fundedBy", locale)} ${logo.alt}` })}
        className="inline-flex items-center gap-tiny rounded-sm px-single py-half text-xs text-muted-foreground hover:text-foreground hover:bg-hover-window transition-colors cursor-pointer no-underline"
      >
        {!iconOnly && <span className="whitespace-nowrap">{demonstratorText("demonstrator.footer.fundedBy", locale)}</span>}
        <span className="inline-flex shrink-0 items-center h-tiny">
          <img src={logo.src} alt="" className="h-full w-auto dark:hidden" />
          {logo.darkSrc !== undefined && <img src={logo.darkSrc} alt="" className="hidden h-full w-auto dark:block" />}
        </span>
      </a>
    ),
  };
}
//#endregion 🏛️FundedByZukunftBauFooterItem

//#region 🏛️AProjectOfLuhUdkFooterItem
/**
 * 🏛️ Left-side footer partner credit mirroring {@link fundedByZukunftBauFooterItem} on the right —
 * "A project of" / "Ein Projekt von" with LUH and UdK logos. Each logo links out to the partner team page;
 * `relative z-40` matches panel-tab chrome so the credit stays visible above anchored panels.
 * 📱️ With `iconOnly` (the landing sets it on a phone) the credit shrinks to just its logos — the caption and "and" have no room.
 **/
export function aProjectOfLuhUdkFooterItem(key: string, locale: DemonstratorLocale, iconOnly: boolean): NavbarItem {
  const [luh, udk] = partnerCredit("aProjectOfLuhUdk", locale).logos;
  return {
    key,
    className: "relative z-40",
    content: (
      <div role="group" aria-label={demonstratorText("demonstrator.footer.aProjectOf", locale)} className="inline-flex items-center gap-tiny rounded-sm px-single py-half text-xs text-muted-foreground hover:text-foreground hover:bg-hover-window transition-colors">
        {!iconOnly && <span className="whitespace-nowrap">{demonstratorText("demonstrator.footer.aProjectOf", locale)}</span>}
        <PartnerLogoLink logo={luh!} locale={locale} />
        {!iconOnly && <span className="whitespace-nowrap">{demonstratorText("demonstrator.footer.and", locale)}</span>}
        <PartnerLogoLink logo={udk!} locale={locale} />
      </div>
    ),
  };
}
//#endregion 🏛️AProjectOfLuhUdkFooterItem
