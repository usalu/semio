// #region 🧲Header

// 🥼 elements/ui/.storybook/nakagin.ts

// Nakagin Capsule Tower example data for Storybook stories (dev-only)
// Sourced from semio/assets/semio/nakagin-capsule-tower.meta.design.semio.json

// #endregion 🧲Header

// #region 📌Design

export const nakagin = {
  name: "Nakagin Capsule Tower",
  description: "The digital shadow of the former Nakagin Capsule Tower which was a mixed-use residential and office tower designed by architect Kisho Kurokawa and located in Shimbashi, Tokyo, Japan. Completed in 1972, the building was a rare remaining example of Japanese Metabolism, an architectural movement emblematic of Japan's postwar cultural resurgence.",
  unit: "m",
} as const;

// #endregion 📌Design

// #region 🎺Architects

export const architects = [
  { id: "1", name: "Kisho Kurokawa", icon: "https://github.com/shadcn.png", role: "Lead Architect", email: "kisho@metabolism.jp" },
  { id: "2", name: "Kenzo Tange", role: "Urban Planner", email: "kenzo@tange.jp" },
  { id: "3", name: "Fumihiko Maki", role: "Architect", email: "fumihiko@maki.jp" },
  { id: "4", name: "Arata Isozaki", role: "Design Director", email: "arata@isozaki.jp" },
  { id: "5", name: "Kiyonori Kikutake", role: "Marine Architect", email: "kiyonori@kikutake.jp" },
] as const;

// #endregion 🎺Architects

// #region ⚙️Types

export const pieceKinds = [
  { id: "capsule", label: "Capsule", icon: "box" },
  { id: "base", label: "Base", icon: "landmark" },
  { id: "tambour", label: "Tambour", icon: "cylinder" },
  { id: "capital", label: "Capital", icon: "crown" },
  { id: "cluster", label: "Cluster", icon: "boxes" },
  { id: "bridge", label: "Bridge", icon: "git-branch" },
] as const;

// #endregion ⚙️Types

// #region 🕹️Properties

export const properties = [
  { id: "gfa", label: "Gross Floor Area", value: "2349.53", unit: "m²" },
  { id: "gwp-intensity", label: "GWP Intensity", value: "22.89", unit: "kgCO2e/m²a" },
  { id: "construction-cost", label: "Construction Cost", value: "2.03e+7", unit: "€" },
  { id: "annual-cost", label: "Annual Cost", value: "2.21e+6", unit: "€/a" },
  { id: "embodied-carbon", label: "Embodied Carbon", value: "1.82e+6", unit: "kgCO2e" },
  { id: "energy-demand", label: "Energy Demand", value: "99.44", unit: "kWh/m²a" },
] as const;

// #endregion 🕹️Properties

// #region 📱Layers

export const layers = [
  { id: "tower", path: "tower" },
  { id: "tower-0", path: "tower/0" },
  { id: "tower-1", path: "tower/1" },
  { id: "tower-2", path: "tower/2" },
  { id: "tower-3", path: "tower/3" },
  { id: "tower-4", path: "tower/4" },
  { id: "tower-5", path: "tower/5" },
  { id: "tower-6", path: "tower/6" },
  { id: "tower-7", path: "tower/7" },
  { id: "tower-8", path: "tower/8" },
  { id: "tower-9", path: "tower/9" },
  { id: "tower-10", path: "tower/10" },
] as const;

// #endregion 📱Layers

// #region 🤖Breadcrumbs

export const breadcrumbs = [
  { label: "Home", href: "/" },
  { label: "Metabolism", href: "/metabolism" },
  { label: "Types", href: "/metabolism/types" },
  { label: "Capsule J", href: "/metabolism/types/capsule-j" },
] as const;

// #endregion 🤖Breadcrumbs
