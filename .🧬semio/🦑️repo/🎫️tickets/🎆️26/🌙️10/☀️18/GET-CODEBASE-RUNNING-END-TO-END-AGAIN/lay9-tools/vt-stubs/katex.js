const katex = { renderToString: (source) => String(source), render: (source, element) => { element.textContent = String(source); } };
export default katex;
export const renderToString = katex.renderToString, render = katex.render;
