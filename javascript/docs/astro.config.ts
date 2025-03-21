// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import react from '@astrojs/react';
import mdx from '@astrojs/mdx';
import markdoc from '@astrojs/markdoc';
import sitemap from '@astrojs/sitemap';
// tailwind is loaded over postcss.config.ts and if loaded again it breaks ⚠️
// import tailwind from '@astrojs/tailwind';
// import tailwindcss from '@tailwindcss/vite';

// https://astro.build/config
export default defineConfig({
    site: 'https://docs.semio-tech.com',
    integrations: [starlight({
        title: {
            en: 'Docs',
            'de': 'Dokumentation',
        },
        defaultLocale: 'root',
        locales: {
            root: {
                label: 'English',
                lang: 'en',
            },
            'de': {
                label: 'Deutsch',
                lang: 'de',
            },
        },
        social: {
            github: 'https://github.com/usalu/semio',
            discord: 'https://discord.gg/m6nnf6pQRc'
        },
        logo: {
            light: './emblem_round.svg',
            dark: './emblem_dark_round.svg',
        },
        editLink: {
            baseUrl: 'https://github.com/usalu/semio/edit/main/javascript/docs',
        },
        sidebar: [
            {
                label: '📌 Start here',
                items: [
                    { label: '🚀 Getting Started', slug: 'getting-started' }
                ],
                translations: {
                    'de': 'Erste Schritte',
                },
            },
            {
                label: '📝 Tutorials',
                autogenerate: { directory: 'tutorials' }

            },
            {
                label: '🌟 Showcases',
                autogenerate: { directory: 'showcases' }
            },
            {
                label: '🦮 Guides',
                items: [
                    { label: '⬇️ Installation', slug: 'installation' },
                    { label: '🥽 Overview', slug: 'overview' }
                ]
            },
            {
                label: '📚 Theory',
                autogenerate: { directory: 'theory' }

            },
            {
                label: '📖 Reference',
                autogenerate: { directory: 'reference' }

            },
        ],
        customCss: ['./globals.css'],
    }), react(), // tailwind({ applyBaseStyles: false, Disable default base styles }),
    mdx(),
    markdoc(),
    sitemap()],
    // vite: {
    // plugins: [tailwindcss()],
    // },
});