import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import react from '@astrojs/react';
import mdx from '@astrojs/mdx';
// import markdoc from '@astrojs/markdoc';
import sitemap from '@astrojs/sitemap';
import tailwindcss from '@tailwindcss/vite';

// https://astro.build/config
export default defineConfig({
    site: 'https://docs.semio-tech.com',
    integrations: [starlight({
        title: {
            en: 'semio docs',
        },
        tableOfContents: { minHeadingLevel: 2, maxHeadingLevel: 4 },
        defaultLocale: 'root',
        locales: {
            root: {
                label: 'English',
                lang: 'en',
            },
            de: {
                label: 'Deutsch',
                lang: 'de',
            },
        },
        social: {
            github: 'https://github.com/usalu/semio',
            discord: 'https://discord.gg/m6nnf6pQRc'
        },
        logo: {
            light: '../../assets/logo/emblem_round.svg',
            dark: '../../assets/logo/emblem_dark_round.svg',
        },
        editLink: {
            baseUrl: 'https://github.com/usalu/semio/edit/main/js/docs',
        },
        sidebar: [
            {
                label: '🚀 Getting Started',
                items: [
                    {
                        label: '🥇 Intro',
                        items: [
                            'intro',
                            'why-semio',
                            'think-in-semio',
                            'core-concepts',
                        ],
                    },
                    'toolkit',
                    'installation',
                    'starter'
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
                label: '🔀 Integrations',
                autogenerate: { directory: 'integrations' }

            },
            {
                label: '📖 Manuals',
                autogenerate: { directory: 'manuals' }

            },
            {
                label: '📚 Theory',
                autogenerate: { directory: 'theory' }

            },
            {
                label: '🌟 Showcases',
                autogenerate: { directory: 'showcases' }
            },
        ],
        customCss: ['./globals.css'],
    }),
    react(),
    mdx(),
    // markdoc(),
    sitemap()],
    vite: {
        plugins: [tailwindcss()],
    },
});