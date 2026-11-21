"""🧭 Read one independent silhouette measurement using its complete source and view identity."""
def issue_lines(row,asset,digest,scale,view):
    rule='volumes-silhouette'
    if (row['id'],row['hash'],row['scale'],row['view'],row['rules'])!=(asset,digest,scale,view,[rule]):
        raise ValueError('Silhouette checkpoint belongs to another input or rule.')
    return [f"VOLUMES 1:{scale} {rule} {item['path']} {view} "+
            ', '.join(f"{m['name']} {m['value']} (limit {m['limit']})" for m in item['measurements'])
            for item in row['findings']]
