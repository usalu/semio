# UI Navbar Umbrella and Barrel Cycle Audit

## Classification

`Navbar/🟦️component.tsx` mixes a coherent primary top-chrome bar with branding, playground selection, and fullscreen measurement facets. The latter responsibilities have independent consumers or ownership and should not remain under the Navbar umbrella.

## Symbol and Consumer Evidence

- `Navbar`, `NavbarProps`, `NavbarItem`: primary top-chrome layout. `Navbar` has active Canvas and OS ShellHost consumers. `NavbarItem` is used by Navbar, Canvas, Footer, and ShellHost.
- `SemioLogo`, `ShellBrandLogo`: branding facet; one active ShellHost terminal. The demonstrator edge is excluded legacy.
- `navbarFillClassName`: zero external consumers and only supports `navbarFillItem`.
- `navbarFillItem`: ShellHost is its one active terminal; retain with Navbar because it is layout-specific.
- `shellNavbarTrailingEndReserveCss`: zero consumers; delete and de-export.
- `NAVBAR_NO_EXAMPLE_ID`, normalization, option, and selector prop contracts are selector-internal.
- `NavbarExampleSelect`: one active ShellHost terminal; its playground/example selection concern should be a dedicated specific component.
- The protected barrel additionally owns `useShellNavbarTrailingEndWidthPx` and `shellNavbarTrailingEndReserveStyle`, used by Panel, plus `NavbarTrailingFullscreenSlot`, used by Navbar.

The OS React package's Navbar imports are unused bridge imports and do not count as terminals.

## Runtime SCC

The current runtime cycle is:

`React barrel -> Navbar -> React barrel`

Navbar imports `NavbarTrailingFullscreenSlot` from the barrel. The barrel's fullscreen region owns a root-keyed ephemeral store, publisher, `useSyncExternalStore` hook, reserve style, and the fullscreen slot. Canvas and Footer add no runtime cycle.

## Lowest Owners and Dispositions

- Fullscreen width state and slot: a dedicated shared UI chrome module because Navbar and Panel are independent consumers.
- Playground selector: a dedicated specific playground/example selector component.
- Branding: a dedicated shell-brand component, currently one ShellHost consumer.
- Primary item contract: ultimately a repository-owned UI chrome item contract. Current `NavbarItem` exposes `React.ReactNode` and `React.Key`; the key can become `string | number`, while opaque content remains at the React adapter boundary.
- Primary Navbar layout remains a specific component.

## Graph-Colored Closures

The green SCC packet avoids protected ShellHost and owns:

- new `ui/🔨️modules/🖥️navbar-fullscreen/🟦️component.tsx`;
- Navbar;
- Panel;
- the serialized React barrel fullscreen/Navbar regions.

Panel requires no API rename if the protected barrel retains the same mechanical export names.

The item-contract rename is red while ShellHost is protected because ShellHost constructs `NavbarItem` directly. Do not combine it with the SCC packet.

## Baseline SHA-256

- Navbar: `2918372bf6dcee1d211db0a0082db4f7cd596db2c06ba6fb91d641d426ce024e`
- Navbar story: `bd3ff5b60d76cf43beac38a3b73b20b2d28ee09266bb47f0b49687608927e134`
- React barrel: `de3c18afdb4a6cb03ef35814457c139547b268d7ba960748ff5bc4c652a52f99`
- Canvas: `f4cce796cb33a41321f1f01790def77408b7d0af8e640d4f7a79bdc7da63aaa4`
- Footer: `ff901f2e47d51a0febffeb0ffdc781476617ec65f44cf0e8e2ae1421f00bd756`
- Panel: `8dd7e066f8646e8fd920c4489c462d0edd0caef98fc076810255dd4a56b06c85`
- ShellHost: `55f0a2b307bc8ab8c292b212f878a4c590dc4b94b09e47bb923f5ef4f879fa3d`
