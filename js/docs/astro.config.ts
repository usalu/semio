import mdx from "@astrojs/mdx";
import react from "@astrojs/react";
import starlight from "@astrojs/starlight";
import { defineConfig } from "astro/config";
// import markdoc from '@astrojs/markdoc';
import sitemap from "@astrojs/sitemap";
import tailwindcss from "@tailwindcss/vite";

// https://astro.build/config
export default defineConfig({
  site: "https://docs.semio-tech.com",
  integrations: [
    starlight({
      title: {
        en: "docs",
      },
      // tableOfContents: { minHeadingLevel: 2, maxHeadingLevel: 3 },
      defaultLocale: "root",
      locales: {
        root: {
          label: "English",
          lang: "en",
        },
        de: {
          label: "Deutsch",
          lang: "de",
        },
      },
      social: [
        {
          icon: "github",
          label: "GitHub",
          href: "https://github.com/usalu/semio",
        },
        {
          icon: "discord",
          label: "Discord",
          href: "https://discord.gg/m6nnf6pQRc",
        },
      ],
      logo: {
        light: "../../assets/logo/emblem_round.svg",
        dark: "../../assets/logo/emblem_dark_round.svg",
      },
      editLink: {
        baseUrl: "https://github.com/usalu/semio/edit/main/js/docs",
      },
      sidebar: [
        {
          label: "🚀 Getting Started",
          items: [
            {
              label: "🥇 Intro",
              items: ["intro", "design-information-modeling", "think-in-semio"],
            },
            "installation",
            "starter",
          ],
          translations: {
            de: "Erste Schritte",
          },
        },
        {
          label: "📝 Tutorials",
          items: [
            {
              label: "👋 Hello semio",
              autogenerate: { directory: "tutorials/hello-semio" },
            },
          ],
        },
        {
          label: "🔀 Integrations",
          autogenerate: { directory: "integrations" },
        },
        {
          label: "📖 Manuals",
          autogenerate: { directory: "manuals" },
        },
        {
          label: "📚 Theory",
          autogenerate: { directory: "theory" },
        },
        {
          label: "🌟 Showcases",
          autogenerate: { directory: "showcases" },
        },
      ],
      components: {
        ThemeSelect: "./src/components/ThemeSelect.astro",
        LanguageSelect: "./src/components/LanguageSelect.astro",
        Search: "./src/components/Search.astro",
        SocialIcons: "./src/components/SocialIcons.astro",
        Sidebar: "./src/components/Sidebar.astro",
        PageSidebar: "./src/components/PageSidebar.astro",
        Header: "./src/components/Header.astro",
        ContentPanel: "./src/components/ContentPanel.astro",
        Banner: "./src/components/Banner.astro",
        DraftContentNotice: "./src/components/DraftContentNotice.astro",
        EditLink: "./src/components/EditLink.astro",
        FallbackContentNotice: "./src/components/FallbackContentNotice.astro",
        Footer: "./src/components/Footer.astro",
        Head: "./src/components/Head.astro",
        Hero: "./src/components/Hero.astro",
        LastUpdated: "./src/components/LastUpdated.astro",
        MarkdownContent: "./src/components/MarkdownContent.astro",
        MobileMenuFooter: "./src/components/MobileMenuFooter.astro",
        MobileMenuToggle: "./src/components/MobileMenuToggle.astro",
        MobileTableOfContents: "./src/components/MobileTableOfContents.astro",
        PageFrame: "./src/components/PageFrame.astro",
        PageTitle: "./src/components/PageTitle.astro",
        Pagination: "./src/components/Pagination.astro",
        SiteTitle: "./src/components/SiteTitle.astro",
        SkipLink: "./src/components/SkipLink.astro",
        TableOfContents: "./src/components/TableOfContents.astro",
        ThemeProvider: "./src/components/ThemeProvider.astro",
        TwoColumnContent: "./src/components/TwoColumnContent.astro",
      },
      customCss: ["./globals.css"],
    }),
    react(),
    mdx(),
    // markdoc(),
    sitemap(),
  ],
  vite: {
    plugins: [tailwindcss()],
  },
});
